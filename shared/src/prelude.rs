pub use crate::{CONFIG, default, string_or_number_serde};
pub use once_cell::sync::Lazy;
pub use serde::{Deserialize, Serialize};
pub use std::collections::{HashMap, HashSet};

pub mod chrono_duration_serde {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::Duration, D::Error> {
		Ok(chrono::Duration::seconds(i64::deserialize(deserializer)?))
	}
	pub fn serialize<S: serde::Serializer>(value: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error> {
		i64::serialize(&value.num_seconds(), serializer)
	}
}

pub mod opt_chrono_duration_serde {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<chrono::Duration>, D::Error> {
		Ok(<Option<i64>>::deserialize(deserializer)?.map(chrono::Duration::seconds))
	}
	pub fn serialize<S: serde::Serializer>(value: &Option<chrono::Duration>, serializer: S) -> Result<S::Ok, S::Error> {
		<Option<i64>>::serialize(&value.map(|x| x.num_seconds()), serializer)
	}
}
