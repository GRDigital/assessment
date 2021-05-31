macro_rules! images {
	($($name:ident => $address:expr),*$(,)*) => {paste::item!{$(
		#[must_use]
		pub fn $name() -> String {
			format!("https://{}/public/img/{}", shared::CONFIG.hostname, $address)
		}
	)*}};
}

images![
	// homepage_frame1 => "homepage/Frame 1.png",
];
