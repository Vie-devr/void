use toml::Value;
use std::fs;
use config::{
	Config as ExternalConfig,
	File,
};
use crate::{
	theme::Theme,
	utils::theme_file,
};

macro_rules! getter {
	($name:ident, $t:ty, $default:expr) => {
		pub fn $name(&self) -> $t {
			self.settings.get::<$t>(stringify!($name)).unwrap_or($default)
		}
	};
}

#[derive(Debug)]
pub struct Config {
	pub settings: ExternalConfig,
	pub theme: Theme,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let settings = ExternalConfig::builder()
						.add_source(File::with_name(path))
						.build()
						.map_err(|_| format!("Config file not found: {}", path))?;

		let theme_name = settings.get_string("theme")
							.map_err(|_| "Theme is not specified")?;
		let theme_path = theme_file(&theme_name);
		let theme = Theme::from_toml(toml_from_file(&theme_path)?)?;

		Ok(Self {
			settings,
			theme,
		})
	}

	getter!(tab_size, usize, 4);
	getter!(text_size, usize, 18);
}

impl Default for Config {
	fn default() -> Self {
		Self {
			settings: ExternalConfig::default(),
			theme: Theme::default(),
		}
	}
}

fn toml_from_file(path: &str) -> Result<Value, String> {
	let content = fs::read_to_string(path)
					.map_err(|_| format!("File not found: {}", path))?;
	let parsed: Value = toml::from_str(&content)
					.map_err(|_| format!("Error parsing file: {}", path))?;

	Ok(parsed)
}
