#![warn(
	clippy::pedantic,
	clippy::clone_on_ref_ptr,
	clippy::decimal_literal_representation,
	clippy::integer_division,
	clippy::todo,
	clippy::wrong_pub_self_convention
)]
#![cfg_attr(not(debug_assertions), warn(
	clippy::dbg_macro,
	clippy::use_debug,
	clippy::print_stdout,
	clippy::unimplemented,
))]
#![allow(
	clippy::missing_panics_doc,
	clippy::cast_precision_loss,
	clippy::module_name_repetitions,
	clippy::default_trait_access,
	clippy::new_without_default,
	clippy::non_ascii_literal,
	clippy::too_many_lines,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::missing_errors_doc,
	clippy::wildcard_imports,
	clippy::doc_markdown,
	dead_code
)]
#![feature(proc_macro_hygiene, try_blocks)]
#![recursion_limit = "1024"]

mod config;
mod prelude;
pub mod api;

pub use config::*;
use prelude::*;
pub use macros;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }

#[macro_export]
macro_rules! enone {
	() => {format!("none {}:{}", file!(), line!())};
}

#[extend::ext(pub)]
impl serde_json::Value {
	fn from_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		self
			.pointer(pointer)
			.context(format!("missing {}", pointer))
			.and_then(|x| Ok(serde_json::from_value(x.clone())?))
	}

	fn from_pointer_mut<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		Ok(self
			.pointer_mut(pointer)
			.map(serde_json::Value::take)
			.map(serde_json::from_value)
			.context(format!("can't extract {}", pointer))??)
	}
}

pub fn string_or_number_serde<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
	use serde::de::Error;

	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Value {
		String(String),
		U64(u64),
	}

	match Value::deserialize(d)? {
		Value::String(x) => x.parse::<u64>().map_err(|e| D::Error::custom(e.to_string())),
		Value::U64(x) => Ok(x),
	}
}

pub trait IntoResOpt: Sized {
	fn into_ok<E>(self) -> Result<Self, E>;
	fn into_some(self) -> Option<Self>;
}

impl<T> IntoResOpt for T {
	fn into_ok<E>(self) -> Result<Self, E> { Ok(self) }
	fn into_some(self) -> Option<Self> { Some(self) }
}

#[macro_export]
macro_rules! enum_inner {
	($e:expr => $path:path) => {if let $path(x) = $e { Some(x) } else { None }};
}
