#![feature(backtrace)]

use directories::ProjectDirs;
use instance::discover_instances;
use lazy_static::lazy_static;

pub mod account;
pub mod component;
pub mod instance;
pub mod java;
pub mod utils;

lazy_static! {
	static ref DIRS: ProjectDirs = ProjectDirs::from("one", "kkx", "mc").unwrap();
	static ref HTTP: reqwest::Client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {
	let instances = discover_instances().await.unwrap();

	for instance in instances {
		println!("java {}", instance.launch().await.unwrap().join(" "));
	}
}
