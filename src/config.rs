use crate::themes;
use macroquad::color::{Color, WHITE};
use phf::Map;
use std::fs;
use toml::{macros::Deserialize, Value};

/// Generates getter method for $name config property of type $t
/// If property is missing, or wrong, generated method will return $default value
macro_rules! getter {
	($name:ident, $t:ty, $default:expr) => {
		pub fn $name(&self) -> $t {
			if let Some(value) = self.settings.get(stringify!($name)) {
				return <$t>::deserialize(value.clone()).unwrap_or($default);
			}

			$default
		}
	};
}

#[derive(Debug)]
pub struct Config {
	settings: Value,
	theme: &'static Map<&'static str, Color>,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let settings = toml_from_file(path)?;
		let theme;

		if let Some(value) = settings.get("theme") {
			let theme_ = value.as_str().ok_or(format!("Wrong theme: {path}"))?;

			theme = themes::THEMES
				.get(theme_)
				.ok_or(format!("Theme {theme_} does not exists"))?;
		}
		else {
			theme = &themes::THEMES["default"];
		}

		Ok(Self { settings, theme })
	}

	pub fn get_color(&self, name: &str) -> Color {
		*self.theme.get(name).unwrap_or(&WHITE)
	}

	getter!(tab_size, usize, 4);
	getter!(text_size, u16, 24);
	getter!(line_nums, bool, true);
}

impl Default for Config {
	fn default() -> Self {
		Self {
			settings: Value::Table(Default::default()),
			theme: &themes::THEMES["default"],
		}
	}
}

pub fn toml_from_file(path: &str) -> Result<Value, String> {
	let content = fs::read_to_string(path).map_err(|_| format!("File {path} not found"))?;
	let parsed: Value = toml::from_str(&content).map_err(|err| format!("{err}: {path}"))?;

	Ok(parsed)
}
