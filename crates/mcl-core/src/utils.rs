use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::time::Duration;

use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use tokio::fs;

use crate::HTTP;

#[derive(Debug, Clone, Copy)]
pub enum Hash {
	Sha1([u8; 20]),
	None(),
}

impl Hash {
	#[must_use]
	pub fn verify(self, data: &[u8]) -> bool {
		match &self {
			// Hash::Sha1(hash) => digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, data).as_ref()
			// == *hash,
			Hash::Sha1(hash) => openssl::sha::sha1(data) == *hash,
			// Hash::Sha1(hash) => Sha1::from(data).digest().bytes() == *hash,
			Hash::None() => true,
		}
	}
}

pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
	if let Some(dir) = path.as_ref().parent() {
		fs::create_dir_all(&dir).await?;
	}
	fs::write(&path, contents).await
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
	#[error("{0}")]
	Reqwest(
		#[from]
		#[backtrace]
		reqwest::Error,
	),
	#[error("{0}")]
	Serde(
		#[from]
		#[backtrace]
		serde_json::Error,
	),
	#[error("{0}")]
	Io(
		#[from]
		#[backtrace]
		io::Error,
	),
	#[error("response hash does not match")]
	HashMismatch(),
	#[error("{0}")]
	SystemTime(
		#[from]
		#[backtrace]
		std::time::SystemTimeError,
	),
}

// somehow split up verifier and downloader fns
//

// read file, verify hash, write file

pub async fn verifier() {}

pub async fn get_json<T: DeserializeOwned>(
	url: impl IntoUrl,
	path: impl AsRef<Path> + std::fmt::Debug,
	hash: Option<Hash>,
	valid_for: Option<Duration>,
) -> Result<T, RequestError> {
	let mut fallback_bytes: Option<Vec<u8>> = None;

	if let Ok(metadata) = fs::metadata(&path).await {
		let file_bytes = fs::read(&path).await?;

		if let Some(hash) = hash {
			if hash.verify(&file_bytes) {
				dbg!(path, "hash matches");
				return Ok(serde_json::from_slice::<T>(&file_bytes)?);
			}
		} else if let Some(valid_for) = valid_for {
			dbg!(metadata.modified()?.elapsed()?, valid_for);
			if metadata.modified()?.elapsed()? <= valid_for {
				dbg!(path, "still valid");
				return Ok(serde_json::from_slice::<T>(&file_bytes)?);
			}
		}

		fallback_bytes = Some(file_bytes);
	}

	match HTTP.get(url).send().await {
		Ok(response) => {
			let response_bytes = response.bytes().await?;

			if let Some(hash) = hash {
				if !hash.verify(&response_bytes) {
					return Err(RequestError::HashMismatch());
				}
			}

			dbg!(&path, "downloaded");
			fs::write(&path, &response_bytes).await?;

			Ok(serde_json::from_slice::<T>(&response_bytes)?)
		},
		Err(err) => {
			if let Some(fallback_bytes) = fallback_bytes {
				dbg!(path, "fallback");
				return Ok(serde_json::from_slice::<T>(&fallback_bytes)?);
			}

			Err(err.into())
		},
	}
}

#[derive(Debug)]
pub enum DownloadResult {
	Downloaded,
	Skipped,
}

pub async fn download_file(
	url: impl IntoUrl,
	path: impl AsRef<Path>,
	hash: Option<Hash>,
	skip: bool,
) -> Result<DownloadResult, RequestError> {
	if skip {
		return Ok(DownloadResult::Skipped);
	}

	if let Some(ref hash) = hash {
		if let Ok(file_bytes) = fs::read(&path).await {
			if hash.verify(&file_bytes) {
				return Ok(DownloadResult::Skipped);
			};
		}
	}

	let bytes = HTTP.get(url).send().await?.bytes().await?;
	write(path, bytes).await?;

	Ok(DownloadResult::Downloaded)
}

pub fn replace_placeholders<H: std::hash::BuildHasher>(
	template: &str,
	variables: &HashMap<String, String, H>,
) -> Option<String> {
	let mut chars = template.chars().peekable();
	let mut out: Vec<char> = Vec::new();
	let mut key: Vec<char> = Vec::new();

	let mut in_placeholder = false;
	while let Some(char) = chars.next() {
		if in_placeholder {
			if char == '}' {
				in_placeholder = false;
				let key_str: String = key.into_iter().collect();
				for char in variables.get(&key_str).or(None)?.chars() {
					out.push(char);
				}
				key = Vec::new();
			} else {
				key.push(char);
			}
		} else if char == '$' && *chars.peek().unwrap_or(&' ') == '{' {
			chars.next();
			in_placeholder = true;
		} else {
			out.push(char);
		}
	}

	Some(out.into_iter().collect())
}
