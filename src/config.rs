use crate::utils::{parse_colorscheme, toml_from_file};
use include_dir::{Dir, include_dir};
use macroquad::color::{Color, WHITE};
use std::collections::HashMap;
use toml::{Value, macros::Deserialize};

const COLORSCHEMES_DIR: Dir = 
	include_dir!("$CARGO_MANIFEST_DIR/res/colorschemes/");

lazy_static::lazy_static! {
	/// Here we are generating HashMap with colorscheme
	/// name as key, and scheme file's content as value
	static ref COLORSCHEMES: HashMap<&'static str, &'static str> =
		COLORSCHEMES_DIR
			.files()
			.map(|file| {
				let name = file.path().file_stem().unwrap().to_str().unwrap();
				let contents = file.contents_utf8().unwrap();

				(name, contents)
			})
			.collect();
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
