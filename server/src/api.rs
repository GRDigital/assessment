#![allow(clippy::needless_pass_by_value)]

use shared::api::*;

use crate::prelude::*;

type Error = anyhow::Error;

example_api_function!({
	log::info!("arg: {}", arg);
	true
});

#[log_derive::logfn(err = "ERROR")]
pub async fn api(req: web::Bytes) -> Result<web::Bytes, crate::error::Error> {
	let mut bytes = web::Buf::reader(req);

	// TODO: this could be deduplicated with another proc macro that would rewrite the function
	match &bincode::deserialize_from::<_, String>(&mut bytes)? as &str {
		example_api_function!(match) => example_api_function!(example_api_function => bytes),
		_ => Err(anyhow::anyhow!("unknown api method").into()),
	}
}
