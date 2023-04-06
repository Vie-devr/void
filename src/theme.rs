use macroquad::color::*;
use toml::Value;

#[derive(Debug)]
pub struct Theme {
	pub background: Color,
	pub foreground: Color,
	pub colors: Vec<Color>,
}

impl Theme {
	pub fn from_toml(toml: Value) -> Result<Self, String> {
		let bg = toml["background"].as_array().ok_or("Invalid background color")?;
		let fg = toml["foreground"].as_array().ok_or("Invalid foreground color")?;
		let colors = toml["colors"].as_array().ok_or("Invalid colors array")?;

		Ok(Self {
			background: toml_vec_to_color(bg),
			foreground: toml_vec_to_color(fg),
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
			background: BLACK,
			foreground: WHITE,
			colors: Vec::new(),
		}
	}
}

#[inline]
fn toml_vec_to_color(vec: &Vec<Value>) -> Color {
	Color::from_rgba(
		vec.get(0).and_then(Value::as_integer).unwrap_or(255) as u8,
		vec.get(1).and_then(Value::as_integer).unwrap_or(255) as u8,
		vec.get(2).and_then(Value::as_integer).unwrap_or(255) as u8,
		vec.get(3).and_then(Value::as_integer).unwrap_or(255) as u8,
	)
}
