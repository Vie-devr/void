use macroquad::{
	prelude::*,
	color::Color,
};

pub struct EditorStyle {
	pub text_params: TextParams,
	pub dimensions: TextDimensions,
	pub text_padding: f32,
	pub line_spacing: f32,
	pub caret_width: f32,
	pub background: Color,
	pub text: Color,
	pub caret: Color,
}

impl EditorStyle {
	pub fn new(text_padding: f32, line_spacing: f32, caret_width: f32,
				font: Font, font_size: u16, font_scale: f32,
				background: Color, text: Color, caret: Color) -> Self {
		Self {
			text_params: TextParams{
				font,
				font_size,
				font_scale,
				font_scale_aspect: 1.0,
				rotation: 0.,
				color: text,
			},
			dimensions: measure_text("█", Some(font), font_size, font_scale),
			text_padding,
			line_spacing,
			caret_width,
			background,
			text,
			caret,
		}
	}
}
