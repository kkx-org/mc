use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::rules::Rule;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
	Snapshot,
	Release,
	OldBeta,
	OldAlpha,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
	pub id: String,
	#[serde(with = "hex::serde")]
	pub sha1: [u8; 20],
	pub size: usize,
	pub total_size: usize,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgumentValue {
	Single(String),
	Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Argument {
	Basic(String),
	Conditional {
		rules: Vec<Rule>,
		value: ArgumentValue,
	},
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
	pub game: Vec<Argument>,
	pub jvm: Vec<Argument>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
	#[serde(with = "hex::serde")]
	pub sha1: [u8; 20],
	pub size: usize,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
	pub client: Download,
	pub client_mappings: Option<Download>,
	pub server: Option<Download>,
	pub server_mappings: Option<Download>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
	pub component: String,
	pub major_version: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryDownloadsArtifact {
	pub artifact: Artifact,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryDownloadsClassifiers {
	pub classifiers: HashMap<String, Artifact>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
	pub path: String,
	#[serde(with = "hex::serde")]
	pub sha1: [u8; 20],
	pub size: usize,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LibraryExtract {
	pub exclude: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Library {
	ExtractNatives {
		name: String,
		rules: Option<Vec<Rule>>,
		downloads: LibraryDownloadsClassifiers,
		extract: Option<LibraryExtract>,
	},
	SingleArtifact {
		name: String,
		rules: Option<Vec<Rule>>,
		downloads: LibraryDownloadsArtifact,
	},
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggingClientFile {
	pub id: String,
	#[serde(with = "hex::serde")]
	pub sha1: [u8; 20],
	pub size: usize,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggingClient {
	pub argument: String,
	pub file: LoggingClientFile,
	pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
	pub client: LoggingClient,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	pub id: String,
	pub arguments: Option<Arguments>,
	pub minecraft_arguments: Option<String>,
	pub asset_index: AssetIndex,
	pub assets: String,
	pub downloads: Downloads,
	pub libraries: Vec<Library>,
	pub logging: Option<Logging>,
	pub java_version: Option<JavaVersion>,
	pub main_class: String,
	pub type_: VersionType,
	pub time: String,
	pub release_time: String,
	pub compliance_level: Option<u8>,
	pub minimum_launcher_version: u8,
}
