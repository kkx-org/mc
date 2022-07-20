use std::env;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
	Allow,
	Disallow,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OsRule {
	pub name: Option<String>,
	pub arch: Option<String>,
	pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureRule {
	pub is_demo_user: Option<bool>,
	pub has_custom_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
	pub action: RuleAction,
	pub os: Option<OsRule>,
	pub features: Option<FeatureRule>,
}

#[must_use]
pub fn match_os_rule(rule: &OsRule) -> bool {
	let os_name = match env::consts::OS {
		"linux" => "linux",
		"macos" => "osx",
		"windows" => "windows",
		_ => "unknown",
	};

	let os_arch = env::consts::ARCH;

	match rule {
		OsRule {
			name: Some(name),
			arch: Some(arch),
			..
		} => name == os_name && arch == os_arch,
		OsRule {
			name: Some(name), ..
		} => name == os_name,
		OsRule {
			arch: Some(arch), ..
		} => arch == os_arch,
		_ => false,
	}
}

#[must_use]
pub fn match_feature_rule(_rule: &FeatureRule) -> bool {
	false
}

#[must_use]
pub fn match_rule(rule: &Rule) -> bool {
	match rule {
		Rule {
			os: Some(os_rule),
			features: Some(feature_rule),
			..
		} => match_os_rule(os_rule) && match_feature_rule(feature_rule),
		Rule {
			os: Some(os_rule), ..
		} => match_os_rule(os_rule),
		Rule {
			features: Some(feature_rule),
			..
		} => match_feature_rule(feature_rule),
		Rule {
			os: None,
			features: None,
			..
		} => true,
	}
}

#[must_use]
pub fn check(rules: &Vec<Rule>) -> bool {
	let mut result = false;

	for rule in rules {
		if match_rule(rule) {
			result = match rule.action {
				RuleAction::Allow => true,
				RuleAction::Disallow => false,
			};
		};
	}
	result
}
