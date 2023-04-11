use crate::{theme::Theme, utils::{theme_file, toml_from_file}};
use config::{Config as ExternalConfig, File};

macro_rules! getter {
	($name:ident, $t:ty, $default:expr) => {
		pub fn $name(&self) -> $t {
			self.settings.get::<$t>(stringify!($name)).unwrap_or($default)
		}
	};
}

#[derive(Debug, Default)]
pub struct Config {
	pub settings: ExternalConfig,
	pub theme: Theme,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let settings = ExternalConfig::builder()
			.add_source(File::with_name(path))
			.build()
			.map_err(|err| format!("Config error: {err}"))?;

		let mut theme = Theme::default();
		if let Ok(theme_name) = settings.get_string("theme") {
			let theme_path = theme_file(&theme_name);
			theme = Theme::from_toml(toml_from_file(&theme_path)?)?;
		}

		Ok(Self { settings, theme })
	}

	getter!(tab_size, usize, 4);
	getter!(text_size, usize, 18);
}
