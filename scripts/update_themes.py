# This file parses all theme files located at res/themes/, and translates 
# them into Rust code, so we don't need to parse them at runtime
# Run this script if you added/changed/deleted any theme
# Note: run only from root of the project, otherwise it will not work properly

from os import listdir
from os.path import isfile, join


def deserialize_themes():
	result = "use macroquad::{color::Color, color_u8};\n"
	result += "use phf::{Map, phf_map};\n\n"
	result += "pub const THEMES: Map<&'static str, Map<&'static str, Color>> = phf_map! {\n"

	for file in listdir("res/themes/"):
		path = join("res/themes/", file)
		if not isfile(path):
			continue

		result += f"\t\"{file}\" => phf_map! {{\n"

		with open(path, "r") as file:
			for line in file.readlines():
				if line == "\n" or line.startswith("#"):
					continue

				splitted = list(filter(lambda s: s != "", line.split(" ")))
				name = splitted[0].strip()
				color = hex_to_color_constructor(splitted[1].strip())

				result += f"\t\t\"{name}\" => {color},\n"

		result += "\t},\n"

	result += "};\n"
	return result

def hex_to_color_constructor(hex):
	hex_byte_count = 2 if len(hex) == 6 else 1

	red = int(hex[0:hex_byte_count], 16)
	green = int(hex[hex_byte_count:(hex_byte_count * 2)], 16)
	blue = int(hex[(hex_byte_count * 2):(hex_byte_count * 3)], 16)

	return f"color_u8!({red}, {green}, {blue}, 255)"

with open("src/themes.rs", "w") as file:
	file.write(deserialize_themes())
