mod fonts;

use crate::prelude::*;
use fonts::fonts;

#[allow(non_upper_case_globals, dead_code)]
pub mod colors {
	pub use super::*;

	macro_rules! def_colors {
		($($name:ident => $color:expr);+$(;)?) => {$(pub static $name: css::Color = css::Color::from_hex($color);)+};
	}

	def_colors! {
		dark => 0x33_33_33_FF;
		debug_blue => 0x00_87_FF_47;
		dim_black => 0x00_00_00_4C;

		checkbox_radio_border => 0x66_66_66_FF;
		caption => 0x9C_9C_9C_FF;
		input_border => 0xDD_DD_DD_FF;
		border_color => 0xE5_E5_E5_FF;
		elem_active => 0xEF_F7_FF_FF;
		elem_hover => 0xF5_F5_F5_FF;

		purple1 => 0x90_13_FE_FF;
		purple2 => 0xAE_3A_FE_FF;
		purple3 => 0xC7_61_FE_FF;
		purple4 => 0xDC_89_FF_FF;
		purple5 => 0xEC_B0_FF_FF;
		purple6 => 0xF8_D8_FF_FF;

		blue1 => 0x00_7A_FF_FF;
		blue2 => 0x2A_97_FF_FF;
		blue3 => 0x55_B1_FF_FF;
		blue4 => 0x80_C9_FF_FF;
		blue5 => 0xAA_DD_FF_FF;
		blue6 => 0xD5_EF_FF_FF;

		red1 => 0xFF_57_56_FF;
		red2 => 0xFF_73_72_FF;
		red3 => 0xFF_8F_8E_FF;
		red4 => 0xFF_AB_AB_FF;
		red5 => 0xFF_C7_C7_FF;
		red6 => 0xFF_E3_E3_FF;

		orange1 => 0xFB_88_32_FF;
		orange2 => 0xFC_9E_54_FF;
		orange3 => 0xFC_B2_76_FF;
		orange4 => 0xFD_C7_98_FF;
		orange5 => 0xFE_DA_BA_FF;
		orange6 => 0xFE_ED_DC_FF;

		green1 => 0x4A_AF_05_FF;
		green2 => 0x69_BC_24_FF;
		green3 => 0x88_CA_47_FF;
		green4 => 0xA7_D7_6F_FF;
		green5 => 0xC6_E4_9A_FF;
		green6 => 0xE3_F2_CB_FF;

		gray1 => 0x66_66_66_FF;
		gray2 => 0x80_80_80_FF;
		gray3 => 0x99_99_99_FF;
		gray4 => 0xB3_B3_B3_FF;
		gray5 => 0xCC_CC_CC_FF;
		gray6 => 0xE6_E5_E6_FF;
	}
}

pub fn style() -> css::Style { fonts() + css::style! {
	html, body {
		css::height!(100 vh),
	}

	body {
		// css::color!(0),
		// css::font_size!(100%),
		// css::font_family!("serif"),
		// css::FontWeight::Normal,
		// css::line_height!(1),
		// css::TextDecorationLine::None,
		// css::Cursor::Auto,
	}

	* {
		css::margin!(0),
		css::padding!(0),
		css::border_width!(0),
		css::border_style!(none),
		css::border_color!(0),
		css::outline_width!(0),
		css::outline_color!(0),

		css::color!(inherit),
		css::FontSize::Inherit,
		css::FontFamily::Inherit,
		css::FontWeight::Inherit,
		css::LineHeight::Inherit,
		css::TextDecorationLine::Inherit,
		css::WhiteSpace::Inherit,

		// css::Cursor::Inherit,
		css::UserSelect::Inherit,

		css::BoxSizing::Initial,
		css::AlignItems::Initial,
		css::TextAlign::Inherit,
		css::background_color!(initial),
	}
} }
