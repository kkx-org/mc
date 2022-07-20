use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
	pub objects: HashMap<String, Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
	#[serde(with = "hex::serde")]
	pub hash: [u8; 20],
	pub size: usize,
}
