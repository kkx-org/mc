use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use path_macro::path;
use serde::{Deserialize, Serialize};

use super::{Component, ComponentEnum, State, Tag, Version};
use crate::utils::{self, download_file, get_json, Hash};
use crate::DIRS;

pub mod arguments;
pub mod asset_index;
pub mod library;
pub mod rules;
pub mod version;
pub mod version_manifest;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MinecraftClient {
	pub version: Version,
}

impl From<Version> for MinecraftClient {
	fn from(version: Version) -> Self {
		Self { version }
	}
}

#[async_trait(?Send)]
impl Component for MinecraftClient {
	async fn install(&self, result: &mut State) -> Result<(), super::Error> {
		let manifest = utils::get_json::<version_manifest::VersionManifest>(
			"https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
			path!(DIRS.data_dir() / "meta" / "minecraft.json"),
			None,
			Some(Duration::from_secs(30 * 60)),
		)
		.await?;

		let partial_version = match &self.version {
			Version::Tag(Tag::Latest) => {
				manifest
					.versions
					.first()
					.ok_or(super::Error::VersionNotFound())?
			},
			Version::Tag(Tag::Stable) => {
				manifest
					.versions
					.iter()
					.find(|version| version.type_ == version_manifest::VersionType::Release)
					.ok_or(super::Error::VersionNotFound())?
			},
			Version::Id(id) => {
				manifest
					.versions
					.iter()
					.find(|version| &version.id == id)
					.ok_or(super::Error::VersionNotFound())?
			},
		};

		let version: version::Version = get_json(
			&partial_version.url,
			path!(DIRS.data_dir() / "versions" / partial_version.id / "meta.json"),
			Some(Hash::Sha1(partial_version.sha1)),
			None,
		)
		.await?;

		let client_jar = path!(DIRS.data_dir() / "versions" / version.id / "client.jar");

		download_file(
			version.downloads.client.url,
			&client_jar,
			Some(Hash::Sha1(version.downloads.client.sha1)),
			false,
		)
		.await?;

		result.classpath.push(client_jar.to_string_lossy().into());

		let tasks = futures::stream::iter(
			version
				.libraries
				.iter()
				.map(|library| library::download(&version.id, library)),
		);

		tasks
			.for_each_concurrent(5, |task| {
				async {
					println!("{:?}", task.await);
				}
			})
			.await;

		for library in version.libraries {
			if let version::Library::SingleArtifact {
				downloads, rules, ..
			} = library
			{
				if rules.is_none() || rules::check(&rules.unwrap()) {
					result.classpath.push(
						path!(DIRS.data_dir() / "libraries" / downloads.artifact.path)
							.to_string_lossy()
							.into(),
					);
				}
			}
		}

		let asset_index: asset_index::AssetIndex = get_json(
			version.asset_index.url,
			path!(
				DIRS.data_dir() / "assets" / "indexes" / format!("{}.json", version.asset_index.id)
			),
			Some(Hash::Sha1(version.asset_index.sha1)),
			None,
		)
		.await?;

		let tasks = futures::stream::iter(asset_index.objects.iter().map(|(_, asset)| {
			let hash_str = hex::encode(&asset.hash);
			let hash_prefix = hash_str.chars().take(2).collect::<String>();

			let url = format!(
				"https://resources.download.minecraft.net/{}/{}",
				hash_prefix, hash_str
			);

			download_file(
				url,
				path!(DIRS.data_dir() / "assets" / "objects" / hash_prefix / hash_str),
				Some(Hash::Sha1(asset.hash)),
				false,
			)
		}));

		tasks
			.for_each_concurrent(30, |task| {
				async {
					println!("{:?}", task.await);
				}
			})
			.await;

		if let Some(arguments) = version.arguments {
			result
				.jvm_arguments
				.extend(arguments::convert(arguments.jvm));
			result
				.game_arguments
				.extend(arguments::convert(arguments.game));
		}

		Ok(())
	}

	fn is_compatible(&self, _component: &ComponentEnum) {}
}
