use macroquad::prelude::*;

pub struct TextDrawer {
	pub font: Font,
}

impl TextDrawer {
	pub fn new(font: Font) -> Self {
		Self {
			font,
		}
	}

	pub fn draw_text(&self, text: &str, x: f32, y: f32, text_size: u16, color: Color) {
		draw_text_ex(
			text,
			x,
			y + text_size as f32,
			TextParams {
				font: self.font,
				font_size: text_size,
				color,
				..Default::default()
			},
		);
	}
}
