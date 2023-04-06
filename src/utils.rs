use std::env;

pub fn config_file() -> String {
	if cfg!(target_os = "linux") {
		return format!("{}/.config/void/config.toml", env::var("HOME").unwrap());
	}

	String::from("config.toml")
}

pub fn theme_file(theme_name: &str) -> String {
	if cfg!(target_os = "linux") {
		return format!("/usr/share/void/themes/{}.toml", theme_name);
	}

	String::from("theme.toml")
}
