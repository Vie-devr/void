use macroquad::prelude::*;
use std::collections::HashMap;

pub struct TextDrawer {
	pub fonts: HashMap<String, Font>,
}

impl TextDrawer {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub fn add_font(&mut self, name: &str, font: Font) {
		self.fonts.insert(String::from(name), font);
	}

	pub fn draw_text(
		&self, text: &str, x: f32, y: f32,
		text_size: u16, color: Color, font_name: &str,
	) {
		draw_text_ex(
			text,
			x,
			y + text_size as f32,
			TextParams {
				font: self.fonts[font_name],
				font_size: text_size,
				font_scale: 1.0,
				color,
				..Default::default()
			},
		);
	}

	pub fn measure_text(
		&self, text: &str,
		text_size: u16, font_name: &str,
	) -> TextDimensions {
		measure_text(text, Some(self.fonts[font_name]), text_size, 1.0)
	}

	pub fn char_width(&self, text_size: u16, font_name: &str) -> f32 {
		self.measure_text("█", text_size, font_name).width
	}
}
