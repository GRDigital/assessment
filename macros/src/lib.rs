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
#![feature(proc_macro_span)]

use proc_macro2::{Ident, Span, TokenStream};
use proc_quote::quote;
use syn::{visit::Visit, visit_mut::VisitMut};

struct AnonReplace {
	vis: syn::Visibility,
	attrs: TokenStream,
	anon_structs: Vec<TokenStream>,
}

impl VisitMut for AnonReplace {
	fn visit_type_mut(&mut self, mut node: &mut syn::Type) {
		match &mut node {
			syn::Type::Macro(syn::TypeMacro { mac }) => {
				if let Some(ident) = mac.path.get_ident() {
					if ident == "anon" {
						let span_hash = {
							use std::hash::{Hash, Hasher};

							let mut hasher = std::collections::hash_map::DefaultHasher::new();
							ident.span().start().line.hash(&mut hasher);
							ident.span().start().column.hash(&mut hasher);
							ident.span().end().line.hash(&mut hasher);
							ident.span().end().column.hash(&mut hasher);
							hasher.finish()
						};
						let struct_name = Ident::new(&format!("Anon{}", span_hash), Span::call_site());
						let attrs = &self.attrs;
						let vis = &self.vis;
						let tokens = &mac.tokens;
						self.anon_structs.push(quote! {
							#[::macros::anon]
							#attrs
							#vis struct #struct_name {
								#tokens
							}
						});
						*node = syn::parse_quote!(#struct_name);
					}
				}
			},
			_ => syn::visit_mut::visit_type_mut(self, node),
		}
	}
}

struct FieldTyper {
	data: Vec<(syn::Ident, syn::Type, syn::Visibility)>,
}

impl<'ast> Visit<'ast> for FieldTyper {
	fn visit_field(&mut self, field: &syn::Field) {
		if let Some(ident) = &field.ident {
			self.data.push((ident.clone(), field.ty.clone(), field.vis.clone()));
		}
	}
}

#[proc_macro_attribute]
pub fn anon(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut item: syn::ItemStruct = syn::parse_macro_input!(item);
	let attrs = &item.attrs;

	let mut visitor = AnonReplace { vis: item.vis.clone(), attrs: quote!(#(#attrs)*), anon_structs: Vec::new() };
	visitor.visit_item_struct_mut(&mut item);
	let anon_structs = &visitor.anon_structs;

	let mut field_typer = FieldTyper { data: Vec::new() };
	field_typer.visit_item_struct(&item);

	let mod_name = quote::format_ident!("{}Fields", item.ident);
	let vis = &item.vis;
	let defs = field_typer.data.iter().map(|(name, ty, vis)| {
		let vis = if matches!(vis, syn::Visibility::Public(_)) { vis.clone() } else { syn::parse_quote!(pub(super)) };
		quote! { #vis type #name = #ty; }
	});
	(quote! {
		#[allow(non_snake_case, non_camel_case_types)]
		#vis mod #mod_name {
			use super::*;
			#(#defs)*
		}

		#(#anon_structs)*
		#item
	}).into()
}

#[proc_macro]
pub fn merge_sig_and_block(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	use syn::spanned::Spanned;

	struct FuckHygiene(proc_macro2::Span);
	impl VisitMut for FuckHygiene {
		fn visit_ident_mut(&mut self, node: &mut syn::Ident) {
			node.set_span(node.span().resolved_at(self.0));
		}
	}

	let mut item: syn::ItemFn = syn::parse_macro_input!(item);
	let mut fuck_hygiene = FuckHygiene(item.block.span());
	fuck_hygiene.visit_item_fn_mut(&mut item);
	proc_quote::ToTokens::to_token_stream(&item).to_string();

	quote!(#item).into()
}

#[proc_macro_attribute]
pub fn api(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	// can't use syn::FnItem because our shared API signatures don't have a body
	#[derive(Debug)]
	struct FnSig {
		ident: syn::Ident,
		inputs: syn::punctuated::Punctuated<syn::FnArg, syn::Token!(,)>,
		output: syn::ReturnType,
	}

	impl syn::parse::Parse for FnSig {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let content;
			input.parse::<syn::Token!(fn)>()?;
			let ident = input.parse()?;
			syn::parenthesized!(content in input);
			let inputs = <syn::punctuated::Punctuated<syn::FnArg, syn::Token!(,)>>::parse_terminated(&content)?;
			let output = input.parse()?;
			input.parse::<syn::Token!(;)>()?;
			Ok(Self { ident, inputs, output })
		}
	}

	impl proc_quote::ToTokens for FnSig {
		fn to_tokens(&self, tokens: &mut TokenStream) {
			let FnSig { ref ident, ref inputs, ref output } = self;
			quote!(fn #ident(#inputs) #output).to_tokens(tokens);
		}
	}

	let item: FnSig = syn::parse_macro_input!(item);

	let global_name = quote::format_ident!("__API__{}", item.ident);
	let name = quote::format_ident!("{}", item.ident);
	// this is used as a persistent identifier of a method that's being run on the server
	// as a consequence, literally moving functions around would break client/server sync aka both have to be updated at the same time
	// which is fine, it also provides a modicum of security vs anyone that would try to exploit our internal api
	let span_info = format!("{}{}", item.ident.span().unwrap().source_file().path().to_string_lossy(), item.ident.span().start().line);
	let args = item.inputs.iter().map(|x| if let syn::FnArg::Typed(x) = x { &x.pat } else { unreachable!() });

	let clientside_serialize_and_reqwest = quote!(
		let mut serialized = Vec::with_capacity((bincode::serialized_size(#span_info)? + bincode::serialized_size(&args)?) as usize);
		bincode::serialize_into(&mut serialized, #span_info)?;
		bincode::serialize_into(&mut serialized, &args)?;
		bincode::deserialize(&CLIENT.post(&API_URL as &str)
			.body(serialized)
			.send().await?.error_for_status()?
			.bytes().await?
		)?
	);

	// serialize and deserialize patterns are useful for functions that take `impl Trait` in an interface while still having a shared concrete implementation
	// e.g. std::ops::RangeBound<T>
	// pattern without an arg is clientside default implementation
	// pattern with a block arg is serverside implementation that allows ommitting function signature entirely,
	// this isn't mandatory and the signature can be written out explicitly
	quote!(
		#[macro_export]
		macro_rules! #global_name {
			() => {
				#[throws]
				pub async #item {
					let args = (#(#args),*);
					#clientside_serialize_and_reqwest
				}
			};
			(serialize $args:tt) => { ::shared::macros::merge_sig_and_block!(
				#[throws]
				pub async #item {
					let args = $args;
					#clientside_serialize_and_reqwest
				}
			); };
			($block:block) => { ::shared::macros::merge_sig_and_block!(#[throws] pub async #item $block); };
			(match) => { #span_info };
			($f:path => $bytes:expr) => { Ok(bincode::serialize(&::std::ops::Fn::call(&$f, bincode::deserialize_from(&mut $bytes)?).await?)?.into()) };
			(deserialize $args:tt $f:path => $bytes:expr) => { Ok(bincode::serialize(&::std::ops::Fn::call(&$f, bincode::deserialize_from::<_, $args>(&mut $bytes)?).await?)?.into()) };
		}

		pub use #global_name as #name;
	).into()
}
