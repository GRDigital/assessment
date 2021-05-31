//# xmltree = { version = "*", features = ["attribute-order"] }
//# structopt = "*"
//# anyhow = "*"

use xmltree::{Element, EmitterConfig, XMLNode};
use std::{
	fs,
	io::{BufReader, BufRead},
};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
	/// Input file
	#[structopt(short, long, parse(from_os_str))]
	input: PathBuf,

	/// Output file
	#[structopt(short, long, parse(from_os_str))]
	output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let opt = Opt::from_args();
	let input = opt.input;
	let output = opt.output.unwrap_or_else(|| input.clone());
	println!("Cleaning {:#?}", &input);

	let svg = std::fs::File::open(&input)?;
	let mut svg_element = Element::parse(svg)?;

	if svg_element.attributes.get("fill").is_none() {
		svg_element.attributes.insert("fill".into(), "none".into());
	}
	clean_element(&mut svg_element);

	// Create and write the output file
	let config = EmitterConfig::new()
		.indent_string("\t")
		.pad_self_closing(false)
		.write_document_declaration(false)
		.keep_element_names_stack(true)
		.perform_indent(true);

	let mut directory = output.clone();
	directory.pop();
	fs::create_dir_all(directory)?;

	let mut out = Vec::new();
	svg_element.write_with_config(&mut out, config)?;
	let mut out = BufReader::new(&*out).lines().skip(1).map(|x| x.unwrap()).collect::<Vec<String>>().join("\n");
	out.push_str("\n");
	fs::write(&output, out)?;

	Ok(())
}

fn clean_element(ele: &mut xmltree::Element) {
	if let Some(attr) = ele.attributes.get_mut("stroke") {
		if attr != "none" {
			*attr = "currentColor".to_string();
		}
	}
	if let Some(attr) = ele.attributes.get_mut("fill") {
		if attr != "none" {
			*attr = "currentColor".to_string();
		}
	}

	for i in 0..ele.children.len() {
		if let Some(XMLNode::Element(child)) = ele.children.get_mut(i) {
			clean_element(child)
		}
	}
}
