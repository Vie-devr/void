use crate::{theme::Theme, utils::{theme_file, toml_from_file}};
use toml::{Value, macros::Deserialize};

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
	pub settings: Value,
	pub theme: Theme,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let settings = toml_from_file(path)?;

		let mut theme = Theme::default();
		if let Some(theme_value) = settings.get("theme") {
			if let Some(theme_name) = theme_value.as_str() {
				let theme_path = theme_file(theme_name);
				theme = Theme::from_toml(toml_from_file(&theme_path)?)?;
			}
		}

		Ok(Self { settings, theme })
	}

	getter!(tab_size, usize, 4);
	getter!(text_size, usize, 18);
}


impl Default for Config {
	fn default() -> Self {
		Self {
			settings: Value::Table(Default::default()),
			theme: Theme::default(),
		}
	}
}
