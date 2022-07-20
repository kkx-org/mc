use std::collections::HashMap;
use std::io;
use std::mem::discriminant;
use std::path::{Path, PathBuf};

use path_macro::path;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::component::minecraft::MinecraftClient;
use crate::component::{self, Argument, Component, ComponentEnum, State, Version};
use crate::utils::replace_placeholders;
use crate::DIRS;

// TODO: Replace all of these error types with a single one
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("instance with this id already exists")]
	InstanceAlreadyExists(),
	#[error("component already exists on this instance")]
	ComponentAlreadyAdded(),
	#[error("io error: {0}")]
	Io(#[from] io::Error),
	#[error("serde json error: {0}")]
	SerdeJson(#[from] serde_json::Error),
	#[error("component error: {0}")]
	ComponentError(#[from] component::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
	id: String,
	components: Vec<ComponentEnum>,
}

impl Instance {
	#[must_use]
	pub fn get_id(&self) -> &String {
		&self.id
	}

	#[must_use]
	pub fn get_path(&self) -> PathBuf {
		path!(DIRS.data_dir() / "instances" / self.id)
	}

	#[must_use]
	pub fn get_components(&self) -> &Vec<ComponentEnum> {
		&self.components
	}

	pub fn add_component(&mut self, component: ComponentEnum) -> Result<(), Error> {
		if self
			.components
			.iter()
			.any(|c| discriminant(&component) == discriminant(c))
		{
			return Err(Error::ComponentAlreadyAdded());
		};

		// check for conflicts

		self.components.push(component);

		Ok(())
	}

	pub async fn rename(&mut self, new_id: String) -> Result<(), Error> {
		let new_path = path!(DIRS.data_dir() / "instances" / new_id);
		if fs::metadata(&new_path).await.is_err() {
			fs::rename(self.get_path(), &new_path).await?;
		}

		self.id = new_id;

		Ok(())
	}

	pub async fn save(&self) -> Result<(), Error> {
		fs::write(
			path!(self.get_path() / "meta.json"),
			serde_json::to_string_pretty(self)?,
		)
		.await?;

		Ok(())
	}

	pub async fn new(id: String, minecraft_version: Version) -> Result<Instance, Error> {
		if fs::metadata(path!(DIRS.data_dir() / "instances" / id))
			.await
			.is_ok()
		{
			return Err(Error::InstanceAlreadyExists());
		};

		fs::create_dir_all(path!(DIRS.data_dir() / "instances" / id)).await?;

		let component: MinecraftClient = minecraft_version.into();

		let instance = Instance {
			id,
			components: vec![ComponentEnum::MinecraftClient(component)],
		};

		Ok(instance)
	}

	pub async fn load(path: impl AsRef<Path>) -> Result<Instance, Error> {
		Ok(serde_json::from_slice::<Instance>(&fs::read(path).await?)?)
	}

	pub async fn install(&self) -> Result<State, Error> {
		let mut result = State {
			classpath: Vec::new(),
			main_class: "net.minecraft.client.main.Main".to_string(),
			jvm_arguments: Vec::new(),
			game_arguments: Vec::new(),
			variables: HashMap::new(),
		};

		for component in self.get_components() {
			component.install(&mut result).await?;
		}

		Ok(result)
	}

	// let _variables = HashMap::from([
	//     ("auth_player_name", "33KK"),
	//     ("version_name", &version.id), // full version
	//     ("game_directory", &root.to_string_lossy()),
	//     ("assets_root", &path!(root / "assets").to_string_lossy()),
	//     ("assets_index_name", "1.19"),
	//     // ("auth_uuid", "a340a0c3-225d-4518-9b47-d660fc50c1ff"),
	//     // ("auth_access_token", ""),
	//     // ("clientid", ""),
	//     // ("auth_xuid", ""),
	//     ("user_type", "offline"),
	//     // ("version_type", ""),
	//     // ("resolution_width", ""),
	//     // ("resolution_height", ""),
	//     (
	//         "natives_directory",
	//         &path!(root / "libraries").to_string_lossy(),
	//     ),
	//     ("launcher_name", "mc"),
	//     ("launcher_version", "0.1"),
	//     ("classpath", &build_classpath(version, root)),
	//     ("path", ""),
	// ]);

	pub async fn launch(&self) -> Result<Vec<String>, Error> {
		let mut result = self.install().await?;

		result
			.variables
			.insert("classpath".to_owned(), result.classpath.join(":"));

		let mut args = Vec::new();
		for argument in result
			.jvm_arguments
			.iter()
			.chain(result.game_arguments.iter())
		{
			match argument {
				Argument::Single(value) => {
					if let Some(value) = replace_placeholders(value, &result.variables) {
						args.push(value);
					}
				},
				Argument::Eq(argument, value) => {
					if let Some(value) = replace_placeholders(value, &result.variables) {
						args.push(format!("{argument}={value}"));
					}
				},
				Argument::Pair(argument, value) => {
					if let Some(value) = replace_placeholders(value, &result.variables) {
						args.push(argument.clone());
						args.push(value);
					}
				},
			}
		}

		Ok(args)
	}
}

pub async fn discover_instances() -> Result<Vec<Instance>, Error> {
	let mut instances: Vec<Instance> = Vec::new();
	let mut dirs = fs::read_dir(path!(DIRS.data_dir() / "instances")).await?;

	while let Ok(Some(dir)) = dirs.next_entry().await {
		instances.push(Instance::load(path!(dir.path() / "meta.json")).await?);
	}

	Ok(instances)
}
