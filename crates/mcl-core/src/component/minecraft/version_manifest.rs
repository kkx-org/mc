use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
	pub latest: Release,
	pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
	pub release: String,
	pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
	Snapshot,
	Release,
	OldBeta,
	OldAlpha,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
	pub id: String,
	pub type_: VersionType,
	pub url: String,
	pub time: String,
	pub release_time: String,
	#[serde(with = "hex::serde")]
	pub sha1: [u8; 20],
	pub compliance_level: u8,
}
