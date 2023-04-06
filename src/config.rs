use crate::{
	theme::Theme,
	utils::theme_file,
};
use toml::Value;
use std::fs;

#[derive(Debug)]
pub struct Config {
	pub tab_size: usize,
	pub text_size: u16,
	pub theme: Theme,
}

impl Config {
	pub fn from_file(path: &str) -> Result<Self, String> {
		let parsed = toml_from_file(path)?;

		let tab_size = match parsed.get("tab_size") {
			Some(value) => value.as_integer().unwrap_or(4) as usize,
			None => 4,
		};
		let text_size = parsed.get("text_size").ok_or("")?
							.as_integer().unwrap_or(28) as u16;

		let theme = match parsed.get("theme") {
			Some(theme) => {
				let path = theme_file(theme.as_str().unwrap_or(""));

				Theme::from_toml(toml_from_file(&path)?)
							.unwrap_or_default()
			},
			None => Theme::default(),
		};

		Ok(Self {
			tab_size,
			text_size,
			theme,
		})
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			tab_size: 4,
			text_size: 28,
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
