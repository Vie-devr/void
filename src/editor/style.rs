use macroquad::{
	prelude::*,
	color::Color,
};

pub struct EditorStyle {
	pub text_params: TextParams,
	pub text_padding: f32,
	pub line_spacing: f32,
	pub font: Font,
	pub font_size: u16,
	pub font_scale: f32,
	pub background: Color,
	pub text: Color,
}

impl EditorStyle {
	pub fn new(text_padding: f32, line_spacing: f32, font: Font, font_size: u16, font_scale: f32, background: Color, text: Color) -> Self {
		Self {
			text_params: TextParams{
				font,
				font_size,
				font_scale,
				font_scale_aspect: 1.,
				rotation: 0.,
				color: text,
			},
			text_padding,
			line_spacing,
			font,
			font_size,
			font_scale,
			background,
			text,
		}
	}

	pub fn measure_text(&self, text: &str) -> TextDimensions {
		measure_text(text, Some(self.font), self.font_size, self.font_scale)
	}
}
