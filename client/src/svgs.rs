use crate::prelude::*;

fn get_svg_element(xml_node: &roxmltree::Node) -> web_sys::SvgElement {
	let node: web_sys::SvgElement = wasm_bindgen::JsCast::unchecked_into(document().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), xml_node.tag_name().name()).unwrap());
	for attribute in xml_node.attributes() {
		node.set_attribute(wasm_bindgen::intern(attribute.name()), attribute.value()).unwrap();
	}
	for child in xml_node.children().filter(roxmltree::Node::is_element) {
		node.append_child(&get_svg_element(&child)).unwrap();
	}
	node
}

macro_rules! svg {
	($($name:ident => $address:expr),*$(,)*) => {$(
		#[must_use]
		pub fn $name() -> cmp::Svg {
			thread_local! { static TEMPLATE: web_sys::SvgElement = get_svg_element(&roxmltree::Document::parse(include_str!($address)).unwrap().root_element()) }
			let element: web_sys::SvgElement = TEMPLATE.with(|x| x.clone_node_with_deep(true).unwrap()).dyn_into().unwrap();
			cmp::Svg(hobo::create::svg_element(&element))
		}
	)*};
}

#[must_use]
pub fn as_url(x: &str) -> String {
	format!(r#"data:image/svg+xml;utf8,{}"#, x.replace('\r', "").replace('\n', "").replace('"', r#"\""#))
}

pub mod maps {
	use super::*;

	svg![
		pin_location => r"../../public/img/icons/maps/map-pin-location.svg",
	];
}

svg![
	// lock => r"../../public/img/icons/lock.svg",
];
