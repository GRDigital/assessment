pub use crate::{api, global_style::*, images, svgs};
pub use anyhow::Context as _;
pub use async_trait::async_trait;
pub use derive_more::*;
pub use futures::prelude::*;
pub use hobo::{create::components as cmp, enclose as e, events, prelude::*, state};
pub use itertools::Itertools as _;
pub use once_cell::sync::Lazy;
pub use serde::{Deserialize, Serialize};
pub use shared::{enone, default, enum_inner};
pub use std::{
	cell::RefCell,
	collections::{BTreeMap, BTreeSet, HashMap},
	convert::TryFrom,
	rc::Rc,
};
pub use wasm_bindgen_futures::spawn_local as spawn;
pub use fehler::{throw, throws};
pub use sugars::*;

pub fn window() -> web_sys::Window { web_sys::window().expect("no window") }
pub fn document() -> web_sys::Document { window().document().expect("no document") }

pub static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
pub static EVENTS: Lazy<events::Events> = Lazy::new(default);

pub fn spawn_complain<T>(x: impl Future<Output = anyhow::Result<T>> + 'static) {
	spawn(async move { if let Err(e) = x.await { log::error!("{:?}", e); } });
}

pub fn breaker() -> cmp::Div {
	cmp::div()
		.class(css::style!(
			.&.& {
				css::flex_basis!(100%),
				css::size!(0),
				css::margin!(0),
			}
		))
}

pub fn set_description(description: &str) {
	thread_local! { static META: web_sys::HtmlMetaElement = document().query_selector("meta[name=description]").unwrap().unwrap().dyn_into().unwrap(); }
	META.with(|x| x.set_content(description));
}

#[macro_export]
macro_rules! dbg {
	() => {
		log::info!("[{}:{}]", file!(), line!());
	};
	($val:expr) => {
		match $val {
			tmp => {
				log::info!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
				tmp
			}
		}
	};
	($val:expr,) => { $crate::dbg!($val) };
	($($val:expr),+ $(,)?) => {
		($($crate::dbg!($val)),+,)
	};
}
