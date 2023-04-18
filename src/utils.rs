use std::env;

#[cfg(target_os = "linux")]
pub fn config_path() -> String {
	format!("{}/.config/void/config.toml", env::var("HOME").unwrap())
}
