use macroquad::color::*;
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
		let bg0 = toml["bg0"].as_array().ok_or("Invalid background color")?;
		let bg1 = toml["bg1"].as_array().ok_or("Invalid background color")?;
		let fg0 = toml["fg0"].as_array().ok_or("Invalid foreground color")?;
		let fg1 = toml["fg1"].as_array().ok_or("Invalid foreground color")?;
		let colors = toml["colors"].as_array().ok_or("Invalid colors array")?;

		Ok(Self {
			bg0: toml_vec_to_color(bg0),
			bg1: toml_vec_to_color(bg1),
			fg0: toml_vec_to_color(fg0),
			fg1: toml_vec_to_color(fg1),
			colors: colors
				.iter()
				.map(|color| toml_vec_to_color(color.as_array().unwrap_or(&Vec::new())))
				.collect(),
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

fn toml_vec_to_color(vec: &[Value]) -> Color {
	let to_int = |x: Option<&Value>| x
		.and_then(Value::as_integer)
		.unwrap_or(255) as u8;

	Color::from_rgba(
		to_int(vec.get(0)),
		to_int(vec.get(1)),
		to_int(vec.get(2)),
		to_int(vec.get(3)),
	)
}
