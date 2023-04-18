//! This file parses all theme files located at res/themes/,and translates 
//! them into Rust code, so we don't need to parse them at runtime
//! Run this script if you added/changed/deleted any theme
//! Note: run only from root of the project, otherwise it will not work properly
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	fs::write("src/themes.rs", deserialize_themes()?)?;

	Ok(())
}

fn deserialize_themes() -> Result<String, Box<dyn Error>> {
	let mut result = String::new();
	result.push_str("use phf::{Map, phf_map};\n");
	result.push_str("use macroquad::{color::Color, color_u8};\n\n");
	result.push_str("pub const THEMES: Map<&'static str, Map<&'static str, Color>> = phf_map! {\n");

	for file in fs::read_dir("res/themes/")? {
		let file = file?;

		result.push_str(&format!(
			"\t\"{}\" => phf_map! {{\n",
			file.path().file_stem().unwrap().to_string_lossy(),
		));

		for line in fs::read_to_string(file.path())?.lines() {
			if line.is_empty() || line.starts_with('#') {
				continue;
			}

			let splitted: Vec<_> =
				line.split_whitespace().map(|s| s.trim()).collect();

			result.push_str(
				&format!("\t\t\"{}\" => {},\n",
				splitted[0],
				hex_to_color_constructor(splitted[1])),
			);
		}

		result.push_str("\t},\n");
	}

	result.push_str("};\n");
	Ok(result)
}

fn hex_to_color_constructor(hex: &str) -> String {
	let hex_byte_count = if hex.len() == 6 { 2 } else { 1 };

	let red = u8::from_str_radix(
		&hex[0..hex_byte_count],
		16,
	).unwrap();
	let green = u8::from_str_radix(
		&hex[hex_byte_count..(hex_byte_count * 2)],
		16,
	).unwrap();
	let blue = u8::from_str_radix(
		&hex[(hex_byte_count * 2)..(hex_byte_count * 3)],
		16,
	).unwrap();

	format!("color_u8!({red}, {green}, {blue}, 255)")
}
