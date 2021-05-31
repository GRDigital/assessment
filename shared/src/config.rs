use crate::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
	pub name: String,
	pub dev: bool,
	pub hostname: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| toml::from_str(include_str!("../../Config.toml")).unwrap());
