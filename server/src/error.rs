use std::fmt::Display;

#[ambassador::delegatable_trait_remote]
trait Display {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>;
}

#[derive(ambassador::Delegate, Debug)]
#[delegate(Display)]
pub struct Error(anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for Error {
	fn from(x: T) -> Self { Self(x.into()) }
}

impl actix_web::error::ResponseError for Error {}
