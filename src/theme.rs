use macroquad::color::{Color, BLACK, WHITE};
use toml::Value;

#[derive(Debug)]
pub struct Theme {
	pub bg0: Color,
	pub bg1: Color,
	pub fg0: Color,
	pub fg1: Color,
	pub colors: Vec<Color>,
}

impl Theme {
	pub fn from_toml(toml: Value) -> Result<Self, String> {
		let toml_to_color =
			|name: &str| hex_to_color(toml[name].as_str().unwrap_or("#fff"));

		let bg0 = toml_to_color("bg0");
		let bg1 = toml_to_color("bg1");
		let fg0 = toml_to_color("fg0");
		let fg1 = toml_to_color("fg1");
		let colors = toml["colors"]
			.as_array()
			.unwrap_or(&Vec::new())
			.iter()
			.map(|color| hex_to_color(color.as_str().unwrap_or("#fff")))
			.collect();

		Ok(Self {
			bg0,
			bg1,
			fg0,
			fg1,
			colors,
		})
	}
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			bg0: BLACK,
			bg1: BLACK,
			fg0: WHITE,
			fg1: WHITE,
			colors: Vec::new(),
		}
	}
}

fn hex_to_color(hex: &str) -> Color {
	let hex = hex.trim_start_matches('#');
	match hex.len() {
		3 => {
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(255);
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(255);
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(255);

			Color::from_rgba(r, g, b, 255)
		}
		6 => {
			let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
			let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
			let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);

			Color::from_rgba(r, g, b, 255)
		}
		_ => WHITE,
	}
}
