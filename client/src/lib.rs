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
	dead_code,

	// TODO:
	clippy::new_ret_no_self,
)]
#![feature(try_blocks, async_closure, bound_cloned)]
#![recursion_limit = "1024"]

#[macro_use]
mod prelude;
pub mod api;
mod global_style;
pub mod images;
pub mod svgs;

use prelude::*;

fn root(element: &web_sys::HtmlElement) {
	spawn_complain(api::example_api_function("WOOOOOO".to_owned()));
	cmp::Body(hobo::create::html_element(element))
		.child(cmp::div()
			.text("Hello world!")
		);
}

#[wasm_bindgen(start)]
pub fn main() {
	console_log::init().unwrap();
	console_error_panic_hook::set_once();

	let body = document().body().unwrap();
	body.set_inner_html("");
	body.set_attribute(web_str::class(), &hobo::fetch_classname(global_style::style())).unwrap();
	root(&body);
}
