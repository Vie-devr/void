use phf::{Map, phf_map};
use macroquad::{color::Color, color_u8};

pub const THEMES: Map<&'static str, Map<&'static str, Color>> = phf_map! {
	"default" => phf_map! {
		"background0" => color_u8!(44, 33, 59, 255),
		"background1" => color_u8!(44, 33, 59, 255),
		"foreground0" => color_u8!(199, 199, 199, 255),
		"foreground1" => color_u8!(199, 199, 199, 255),
		"ident" => color_u8!(15, 15, 15, 255),
		"keyword" => color_u8!(15, 15, 15, 255),
		"type" => color_u8!(15, 15, 15, 255),
		"string" => color_u8!(15, 15, 15, 255),
		"comment" => color_u8!(15, 15, 15, 255),
	},
};
