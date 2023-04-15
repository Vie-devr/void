use crate::utils::{parse_colorscheme, toml_from_file};
use macroquad::color::{Color, WHITE};
use std::collections::HashMap;
use toml::{Value, macros::Deserialize};

lazy_static::lazy_static! {
	static ref COLORSCHEMES: HashMap<&'static str, &'static str> =
		HashMap::from([
			("default", include_str!("../res/themes/default.void")),
		]);
}

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
	colorscheme: HashMap<String, Color>,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let settings = toml_from_file(path)?;
		let colorscheme;

		if let Some(value) = settings.get("theme") {
			let scheme = value
				.as_str()
				.ok_or(format!("Config error: Wrong theme: {path}"))?;

			colorscheme = parse_colorscheme(
				COLORSCHEMES
					.get(scheme)
					.ok_or(format!("Config error: theme {scheme} does not exists"))?
			);
		}
		else {
			colorscheme = parse_colorscheme(COLORSCHEMES["default"]);
		}

		Ok(Self { settings, colorscheme })
	}

	pub fn get_color(&self, name: &str) -> Color {
		*self.colorscheme.get(name).unwrap_or(&WHITE)
	}

	getter!(tab_size, usize, 4);
	getter!(text_size, usize, 24);
}


impl Default for Config {
	fn default() -> Self {
		Self {
			settings: Value::Table(Default::default()),
			colorscheme: parse_colorscheme(COLORSCHEMES["default"]),
		}
	}
}
