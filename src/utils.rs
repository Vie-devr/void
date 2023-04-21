use std::env;

#[cfg(target_os = "linux")]
pub fn config_path() -> String {
	format!("{}/.config/void/config.toml", env::var("HOME").unwrap())
}

#[cfg(target_os = "windows")]
pub fn config_path() -> String {
	format!("{}/Void/config.toml", env::var("APPDATA").unwrap())
}
