use std::collections::HashMap;

use async_trait::async_trait;
use authlib_injector::AuthlibInjector;
use enum_dispatch::enum_dispatch;
use minecraft::MinecraftClient;
use serde::{Deserialize, Serialize};

use crate::utils::RequestError;

pub mod authlib_injector;
pub mod fabric;
pub mod minecraft;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
	Latest,
	Stable,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Version {
	Tag(Tag),
	Id(String),
}

// TODO: Replace all of these error types with a single one
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("request error: {0}")]
	Request(#[from] RequestError),
	#[error("reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),
	#[error("tagged version not found")]
	VersionNotFound(),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Argument {
	Single(String),
	Eq(String, String),
	Pair(String, String),
}

pub struct State {
	pub main_class: String,
	pub classpath: Vec<String>,
	pub variables: HashMap<String, String>,
	pub game_arguments: Vec<Argument>,
	pub jvm_arguments: Vec<Argument>,
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "id", rename_all = "kebab-case")]
pub enum ComponentEnum {
	MinecraftClient,
	AuthlibInjector,
}

#[async_trait(?Send)]
#[enum_dispatch(ComponentEnum)]
pub trait Component {
	fn is_compatible(&self, component: &ComponentEnum);
	async fn install(&self, output: &mut State) -> Result<(), Error>;
}
