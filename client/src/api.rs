use crate::prelude::*;

type Error = anyhow::Error;

static API_URL: Lazy<String> = Lazy::new(|| format!("https://{}/api", shared::CONFIG.hostname));

shared::api::example_api_function!();
