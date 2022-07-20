use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{Component, ComponentEnum, State, Version};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthlibInjector {
	pub version: Version,
}

#[async_trait(?Send)]
impl Component for AuthlibInjector {
	async fn install(&self, _result: &mut State) -> Result<(), super::Error> {
		Ok(())
	}

	fn is_compatible(&self, _component: &ComponentEnum) {}
}
