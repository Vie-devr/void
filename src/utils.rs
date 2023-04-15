use macroquad::color::Color;
use std::{collections::HashMap, env, fs};
use toml::{Value};

#[cfg(target_os = "linux")]
pub fn config_path() -> String {
	format!("{}/.config/void/config.toml", env::var("HOME").unwrap())
}

pub fn toml_from_file(path: &str) -> Result<Value, String> {
	let content = fs::read_to_string(path)
		.map_err(|_| format!("File error: File {path} not found"))?;
	let parsed: Value = toml::from_str(&content)
		.map_err(|err| format!("Parsing error: {err}: {path}"))?;

	Ok(parsed)
}

pub fn parse_colorscheme(scheme: &str) -> HashMap<String, Color> {
	let mut result = HashMap::new();

	for line in scheme.lines() {
		let line = line.trim();
		if line.is_empty() || line.starts_with('#') {
			continue;
		}

		let splitted: Vec<&str> = line.split_whitespace().collect();
		result.insert(
			splitted[0].to_string(),
			hex_to_color(splitted[1]),
		);
	}

	result
}

fn hex_to_color(hex: &str) -> Color {
	let hex = hex.trim_start_matches('#');

	match hex.len() {
		3 => {
			let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap();
			let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap();
			let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap();

			Color::from_rgba(r, g, b, 255)
		}
		6 => {
			let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
			let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
			let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

			Color::from_rgba(r, g, b, 255)
		}
		_ => unreachable!(),
	}
}
