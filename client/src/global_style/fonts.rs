use crate::prelude::*;

pub fn fonts() -> css::Style { css::style! {
	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEz0dL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEzQdL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEzwdL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEzMdL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEz8dL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEz4dL_nz.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin Italic".into()), Source::Local("Roboto-ThinItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOiCnqEu92Fr1Mu51QrEzAdLw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc3CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc-CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc2CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc5CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc1CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc0CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light Italic".into()), Source::Local("Roboto-LightItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TjASc6CsQ.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xFIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xMIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xEIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xLIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xHIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xGIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto Italic".into()), Source::Local("Roboto-Italic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1Mu51xIIzI.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc3CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc-CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc2CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc5CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc1CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc0CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium Italic".into()), Source::Local("Roboto-MediumItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51S7ACc6CsQ.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic3CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic-CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic2CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic5CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic1CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic0CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold Italic".into()), Source::Local("Roboto-BoldItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TzBic6CsQ.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc3CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc-CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc2CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc5CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc1CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc0CsTKlA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Italic,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black Italic".into()), Source::Local("Roboto-BlackItalic".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOjCnqEu92Fr1Mu51TLBCc6CsQ.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxFIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxMIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxEIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxLIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxHIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxGIzIFKw.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(100), None),
		src: vec![Source::Local("Roboto Thin".into()), Source::Local("Roboto-Thin".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOkCnqEu92Fr1MmgVxIIzI.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fCRc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fABc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fCBc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fBxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fCxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fChc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(300), None),
		src: vec![Source::Local("Roboto Light".into()), Source::Local("Roboto-Light".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmSU5fBBc4.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu72xKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu5mxKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu7mxKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu4WxKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu7WxKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu7GxKOzY.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(400), None),
		src: vec![Source::Local("Roboto".into()), Source::Local("Roboto-Regular".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOmCnqEu92Fr1Mu4mxK.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fCRc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fABc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fCBc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fBxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fCxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fChc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(500), None),
		src: vec![Source::Local("Roboto Medium".into()), Source::Local("Roboto-Medium".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmEU9fBBc4.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfCRc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfABc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfCBc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfBxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfCxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfChc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(700), None),
		src: vec![Source::Local("Roboto Bold".into()), Source::Local("Roboto-Bold".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmWUlfBBc4.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfCRc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0460..0x052F, 0x1C80..0x1C88, 0x20B4, 0x2DE0..0x2DFF, 0xA640..0xA69F, 0xFE2E..0xFE2F],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfABc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0400..0x045F, 0x0490..0x0491, 0x04B0..0x04B1, 0x2116],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfCBc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x1F00..0x1FFF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfBxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0370..0x03FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfCxc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0102..0x0103, 0x0110..0x0111, 0x0128..0x0129, 0x0168..0x0169, 0x01A0..0x01A1, 0x01AF..0x01B0, 0x1EA0..0x1EF9, 0x20AB],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfChc4EsA.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0100..0x024F, 0x0259, 0x1E00..0x1EFF, 0x2020, 0x20A0..0x20AB, 0x20AD..0x20CF, 0x2113, 0x2C60..0x2C7F, 0xA720..0xA7FF],
		display: Display::Swap,
	}

	@font-face {
		family: "Roboto".into(),
		style: Style::Normal,
		weight: (Weight::Number(900), None),
		src: vec![Source::Local("Roboto Black".into()), Source::Local("Roboto-Black".into()), Source::Url("https://fonts.gstatic.com/s/roboto/v20/KFOlCnqEu92Fr1MmYUtfBBc4.woff2".into(), Some(Format::Woff2))],
		unicode_range: unicode_range![0x0000..0x00FF, 0x0131, 0x0152..0x0153, 0x02BB..0x02BC, 0x02C6, 0x02DA, 0x02DC, 0x2000..0x206F, 0x2074, 0x20AC, 0x2122, 0x2191, 0x2193, 0x2212, 0x2215, 0xFEFF, 0xFFFD],
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-Regular.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-ExtraBold.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		weight: (Weight::Number(800), None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-Bold.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		weight: (Weight::Number(700), None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-Light.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		weight: (Weight::Number(300), None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-ExpandedMedium.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		weight: (Weight::Number(500), None),
		stretch: (Stretch::Expanded, None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url(format!("https://{}/public/fonts/Fixture-ExpandedExtraBold.ttf", &shared::CONFIG.hostname), Some(Format::TrueType))],
		family: "Fixture".into(),
		weight: (Weight::Number(800), None),
		stretch: (Stretch::Expanded, None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2".into(), None)],
		family: "Montserrat".into(),
		weight: (Weight::Normal, None),
		display: Display::Swap,
	}

	@font-face {
		src: vec![Source::Url("https://fonts.gstatic.com/s/encodesans/v4/LDI2apOFNxEwR-Bd1O9uYPOreec.woff2".into(), Some(Format::Woff2))],
		family: "Encode Sans".into(),
		weight: (Weight::Number(400), None),
		display: Display::Swap,
	}
} }
