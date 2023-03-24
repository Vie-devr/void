use macroquad::{
	prelude::*,
	color::Color,
};

pub struct EditorStyle {
	pub tab_size: usize,
	pub text_params: TextParams,
	pub dimensions: TextDimensions,
	pub text_padding: f32,
	pub line_spacing: f32,
	pub caret_width: f32,
	pub background: Color,
	pub line_nums_background: Color,
	pub line_nums_text: Color,
	pub text: Color,
	pub caret: Color,
}

impl EditorStyle {
	pub fn new(tab_size: usize, text_padding: f32, line_spacing: f32, caret_width: f32,
				font: Font, font_size: u16, font_scale: f32,
				background: Color, text: Color, caret: Color,
				line_nums_background: Color, line_nums_text: Color) -> Self {
		Self {
			tab_size,
			text_params: TextParams{
				font,
				font_size,
				font_scale,
				font_scale_aspect: 1.0,
				rotation: 0.0,
				color: text,
			},
			dimensions: measure_text("█", Some(font), font_size, font_scale),
			text_padding,
			line_spacing,
			caret_width,			
			background,
			line_nums_background,
			text,
			line_nums_text,
			caret,
		}
	}

	pub fn draw_text(&self, text: &str, x: f32, y: f32, color: Color) {
		draw_text_ex(
			text,
			x,
			y,
			TextParams {
				font: self.text_params.font,
				font_size: self.text_params.font_size,
				font_scale: self.text_params.font_scale,
				font_scale_aspect: self.text_params.font_scale_aspect,
				rotation: self.text_params.rotation,
				color,
			},
		)
	}


	pub fn measure_text(&self, text: &str) -> TextDimensions {
		measure_text(
			text,
			Some(self.text_params.font),
			self.text_params.font_size,
			self.text_params.font_scale,
		)
	}
}
