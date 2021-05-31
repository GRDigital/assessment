#![feature(try_blocks, async_closure, fn_traits)]

mod api;
mod error;
mod prelude;

use prelude::*;

fn reply() -> web::HttpResponse {
	static REPLY: Lazy<String> = Lazy::new(|| {
		let js = minifier::js::minify(&std::fs::read_to_string("public/wasm/client.js").expect("couldn't find the js payload"));

		minify::html::minify(&format!(
			include_str!("../response.html"),
			js = js,
			path = format!("https://{}/public/wasm/client_bg.wasm", shared::CONFIG.hostname),
		))
	});

	web::HttpResponse::Ok().body(&REPLY as &str)
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
	env_logger::builder()
		.filter(None, log::LevelFilter::Info)
		.filter(Some("server"), #[cfg(debug_assertions)] log::LevelFilter::Trace, #[cfg(not(debug_assertions))] log::LevelFilter::Debug)
		.format(|buf, record| {
			use std::io::Write;

			let mut dimmed = buf.style();
			dimmed.set_color(env_logger::fmt::Color::Rgb(126, 126, 126));

			let mut level_style = buf.style();
			match record.level() {
				log::Level::Trace => &mut dimmed,
				log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
				log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red),
				_ => &mut level_style,
			};

			writeln!(buf, "[{time} {level}  {module} {file}:{line}] {args}",
				time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
				level = level_style.value(record.level()),
				module = record.module_path().unwrap_or("module?"),
				file = dimmed.value(record.file().unwrap_or("file?")),
				line = dimmed.value(record.line().unwrap_or(0)),
				args = record.args(),
			)
		})
		.init();

	actix_web::HttpServer::new(|| {
		use actix_web::{http::ContentEncoding, middleware::*};
		use actix_files::{Files, NamedFile};

		actix_web::App::new()
			.wrap(Logger::new("%s in %Ts, %b bytes \"%r\""))
			.wrap(NormalizePath::default())
			.wrap(Compress::new(ContentEncoding::Auto))

			// files
			.service(web::resource("/favicon.ico").to(async || NamedFile::open("public/img/favicon.ico")))
			.service(web::resource("/sitemap.xml").to(async || NamedFile::open("public/sitemap.xml")))
			.service(web::resource("/robots.txt").to(async || NamedFile::open("public/robots.txt")))
			.service(Files::new("/public", "public"))

			// 2^25 bytes aka 32 mb
			.service(web::resource("/api").app_data(web::PayloadConfig::new(1 << 25)).to(api::api))

			// rest
			.default_service(web::route().to(reply))
	})
		.bind_rustls(format!("0.0.0.0:{}", CONFIG.port), {
			use rustls::{ServerConfig, NoClientAuth, internal::pemfile::{certs, pkcs8_private_keys}};
			use std::{io::BufReader, fs::File};

			let mut config = ServerConfig::new(NoClientAuth::new());
			let cert_chain = certs(&mut BufReader::new(File::open(&CONFIG.ssl.cert)?)).ok().context("no certs")?;
			let mut keys = pkcs8_private_keys(&mut BufReader::new(File::open(&CONFIG.ssl.key)?)).ok().context("no private keys")?;
			config.set_single_cert(cert_chain, keys.remove(0))?;
			config
		})?
		.run().await?
		.into_ok()
}
