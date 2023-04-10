use macroquad::prelude::*;

pub struct TextDrawer {
	pub font: Font,
}

impl TextDrawer {
	pub const fn new(font: Font) -> Self {
		Self { font }
	}

	pub fn draw_text(&self, text: &str, x: f32, y: f32, text_size: u16, color: Color) {
		draw_text_ex(
			text,
			x,
			y + text_size as f32,
			TextParams {
				font: self.font,
				font_size: text_size,
				font_scale: 1.0,
				color,
				..Default::default()
			},
		);
	}

	pub fn measure_text(&self, text: &str, text_size: u16) -> TextDimensions {
		measure_text(text, Some(self.font), text_size, 1.0)
	}
}
