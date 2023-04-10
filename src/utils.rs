use std::{env, fs::read_to_string};
use toml::{from_str, Value};

pub fn config_file() -> String {
	match env::consts::OS {
		"linux" => format!("{}/.config/void/config.toml", env::var("HOME").unwrap()),
		_ => unreachable!(),
	}
}

pub fn theme_file(theme_name: &str) -> String {
	match env::consts::OS {
		"linux" => format!("/usr/share/void/themes/{}.toml", theme_name),
		_ => unreachable!(),
	}
}

pub fn root_dir() -> String {
	match env::consts::OS {
		"linux" => env::var("HOME").unwrap(),
		_ => unreachable!(),
	}
}

pub fn toml_from_file(path: &str) -> Result<Value, String> {
	let content = read_to_string(path).map_err(|_| format!("File not found: {}", path))?;
	let parsed: Value = from_str(&content).map_err(|_| format!("Error parsing file: {}", path))?;

	Ok(parsed)
}
