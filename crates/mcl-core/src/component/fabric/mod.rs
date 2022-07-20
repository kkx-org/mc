use serde::{Deserialize, Serialize};

use super::Version;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FabricLoader {
	pub version: Version,
}

impl From<Version> for FabricLoader {
	fn from(version: Version) -> Self {
		Self { version }
	}
}

// #[async_trait(?Send)]
// impl Component for FabricLoaderComponent {
//     async fn install(&self, _: &Instance) -> Result<InstallResult,
// ComponentError> {         // let manifest =
// self::version_manifest::get(&crate::HTTP).await?;         //
//         // let partial_version = match &self.version {
//         //     Version::Tag(Tag::Latest) => manifest
//         //         .versions
//         //         .first()
//         //         .ok_or(ComponentError::VersionNotFound())?,
//         //     Version::Tag(Tag::Stable) => manifest
//         //         .versions
//         //         .iter()
//         //         .find(|version| version.type_ ==
// self::version_manifest::VersionType::Release)         //
// .ok_or(ComponentError::VersionNotFound())?,         //     Version::Id(id) =>
// manifest         //         .versions
//         //         .iter()
//         //         .find(|version| &version.id == id)
//         //         .ok_or(ComponentError::VersionNotFound())?,
//         // };
//     }
//
//     fn is_compatible(&self, _component: &AnyComponent) {}
// }
