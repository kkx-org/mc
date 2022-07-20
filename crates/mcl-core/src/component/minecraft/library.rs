use std::{env, io};

use path_macro::path;
use zip::read::ZipArchive;

use super::rules;
use super::version::Library;
use crate::utils::{self, Hash, RequestError};
use crate::DIRS;

pub async fn download(version_id: &String, library: &Library) -> Result<(), RequestError> {
	match library {
		Library::ExtractNatives {
			downloads,
			rules,
			extract,
			..
		} => {
			if let Some(rules) = rules {
				if !rules::check(rules) {
					return Ok(());
				}
			};

			let artifact = match env::consts::OS {
				"linux" => {
					downloads
						.classifiers
						.get("natives-linux")
						.expect("this shouldn't ever happen")
				},
				"macos" => {
					downloads
						.classifiers
						.get("natives-osx")
						.expect("this shouldn't ever happen")
				},
				"windows" => {
					downloads
						.classifiers
						.get("natives-windows")
						.expect("this shouldn't ever happen")
				},
				_ => todo!(),
			};

			let artifact_path = path!(DIRS.data_dir() / "libraries" / artifact.path);

			utils::download_file(
				&artifact.url,
				&artifact_path,
				Some(Hash::Sha1(artifact.sha1)),
				false,
			)
			.await?;

			let mut jar = ZipArchive::new(std::fs::File::open(artifact_path)?).unwrap();

			for i in 0..jar.len() {
				let mut file = jar.by_index(i).unwrap();

				if file.is_file()
					&& !match extract {
						Some(extract) => {
							extract
								.exclude
								.iter()
								.any(|exclude| file.name().starts_with(exclude))
						},
						None => file.name().starts_with("META-INF/"),
					} {
					let mut buf: Vec<u8> = Vec::new();
					io::copy(&mut file, &mut buf).unwrap();

					utils::write(
						path!(
							DIRS.data_dir()
								/ "versions" / version_id / "natives"
								/ file.enclosed_name().unwrap()
						),
						buf,
					)
					.await
					.unwrap();
				};
			}
		},
		Library::SingleArtifact {
			downloads, rules, ..
		} => {
			utils::download_file(
				downloads.artifact.url.clone(),
				path!(DIRS.data_dir() / "libraries" / downloads.artifact.path),
				Some(Hash::Sha1(downloads.artifact.sha1)),
				rules.is_some() && !rules::check(rules.as_ref().unwrap()),
			)
			.await?;
		},
	};

	Ok(())
}
