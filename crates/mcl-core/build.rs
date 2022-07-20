use std::process::{self, Command, Stdio};

pub fn main() {
	println!("cargo:rerun-if-changed=java/src");

	process::exit(
		Command::new("mvn")
			.arg("package")
			.current_dir(std::fs::canonicalize("./java").unwrap())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.spawn()
			.unwrap()
			.wait()
			.unwrap()
			.code()
			.unwrap(),
	);
}
