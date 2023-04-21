mod gap_buffer;
mod text_drawer;

use crate::config::Config;
use gap_buffer::GapBuffer;
use macroquad::{miniquad::CursorIcon, prelude::*};
use std::ops::Range;
use text_drawer::TextDrawer;

const FONT_REGULAR: &[u8] =
	include_bytes!("../../res/fonts/jet_brains_mono_regular.ttf");
const FONT_SEMI_BOLD: &[u8] =
	include_bytes!("../../res/fonts/jet_brains_mono_semi_bold.ttf");
const FONT_BOLD: &[u8] =
	include_bytes!("../../res/fonts/jet_brains_mono_extra_bold.ttf");

#[derive(Debug)]
struct Line {
	start: usize,
	end: usize,
}

impl Line {
	fn new(start: usize, end: usize) -> Self {
		Self {
			start,
			end,
		}
	}

	fn to_range(&self) -> Range<usize> {
		self.start..self.end
	}
}

#[derive(Debug)]
pub struct Editor {
	buffer: GapBuffer,
	lines: Vec<Line>,
	drawer: TextDrawer,
	caret: usize,
}

impl Editor {
	pub fn new() -> Self {
		let mut drawer = TextDrawer::new();
		drawer.add_font("main", load_ttf_font_from_bytes(FONT_SEMI_BOLD).unwrap());
		drawer.add_font("line_nums", load_ttf_font_from_bytes(FONT_REGULAR).unwrap());
		drawer.add_font("bold", load_ttf_font_from_bytes(FONT_BOLD).unwrap());

		let mut buffer = GapBuffer::new();
		buffer.insert_char('\n', 0);

		Self {
			buffer,
			lines: vec![Line::new(0, 0)],
			caret: 0,
			drawer,
		}
	}

	pub fn process_input(&mut self, key: KeyCode, chr: Option<char>) {
		let ctrl_pressed = is_key_down(KeyCode::LeftControl);

		match key {
			// Paste text from clipboard
			KeyCode::V if ctrl_pressed => {
				let context = unsafe { get_internal_gl().quad_context };

				if let Some(content) = context.clipboard_get() {
					let len = content.chars().count();

					self.buffer.insert(&content, self.caret);
					self.caret += len;
				}
			}
			// Move caret one word right
			KeyCode::Right if ctrl_pressed => {
				while self.caret < self.buffer.len()
					&& !self.buffer.at(self.caret).is_alphabetic() {
					self.caret += 1;
				}

				while self.caret < self.buffer.len()
					&& self.buffer.at(self.caret).is_alphabetic() {
					self.caret += 1;
				}
			}
			// Move caret one word left
			KeyCode::Left if ctrl_pressed => {
				while self.caret > 0
					&& !self.buffer.at(self.caret).is_alphabetic() {
					self.caret -= 1;
				}

				while self.caret > 0
					&& self.buffer.at(self.caret - 1).is_alphabetic() {
					self.caret -= 1;
				}
			}
			// Move caret one char right
			KeyCode::Right => {
				if self.caret < self.buffer.len() - 1 {
					self.caret += 1;
				}
			}
			// Move caret one char left
			KeyCode::Left => {
				if self.caret > 0 {
					self.caret -= 1;
				}
			}
			// Print new line
			KeyCode::Enter => {
				self.buffer.insert_char('\n', self.caret);
				self.caret += 1;
			}
			// Print tabulation
			KeyCode::Tab => {
				self.buffer.insert_char('\t', self.caret);
				self.caret += 1;
			}
			// Delete char before caret
			KeyCode::Backspace => {
				if self.caret > 0 {
					self.buffer.delete_char(self.caret - 1);
					self.caret -= 1;
				}
			}
			// Delete char after caret
			KeyCode::Delete => {
				if self.caret < self.buffer.len() - 1 {
					self.buffer.delete_char(self.caret);
				}
			}
			// Print char
			_ => {
				if let Some(chr) = chr {
					if chr.is_ascii() || chr.is_alphabetic() {
						self.buffer.insert_char(chr, self.caret);
						self.caret += 1;
					}
				}
			}
		}

		self.update_lines();
	}

	pub fn draw(&self, config: &Config) {
		let chars = self.buffer.to_vec();
		let text_size = config.text_size();
		let char_width = self.drawer.char_width(text_size, "main");
		let line_nums_width =
			(self.lines.len().to_string().len() + 2) as f32 * char_width;

		// Draw editor background
		clear_background(config.get_color("background0"));

		// Draw line numbers background
		draw_rectangle(
			0.0,
			0.0,
			line_nums_width,
			screen_height(),
			config.get_color("background1"),
		);

		for (i, line) in self.lines.iter().enumerate() {
			let y = (i * text_size as usize) as f32;
			let line_num = &(i + 1).to_string();
			let line_num_x =
				line_nums_width - (line_num.len() as f32 + 1.0) * char_width;

			// Draw line number
			self.drawer.draw_text(
				line_num,
				line_num_x,
				y,
				text_size,
				config.get_color("foreground0"),
				"line_nums",
			);

			let mut j = 0;
			for chr in &chars[line.to_range()] {
				if chr == &'\t' {
					j += config.tab_size();
					continue;
				}

				let x = j as f32 * char_width + line_nums_width;

				// Draw char
				self.drawer.draw_text(
					&chr.to_string(),
					x,
					y,
					text_size,
					config.get_color("foreground0"),
					"main",
				);

				j += 1;
			}
		}

		Self::update_mouse_cursor();
	}
}

impl Editor {
	fn update_mouse_cursor() {
		let context = unsafe { get_internal_gl().quad_context };
		let mouse_pos = mouse_position();

		// Set mouse cursor to "Text", when it is over the editor
		if mouse_pos.0 >= 0.0 && mouse_pos.0 <= screen_width()
		&& mouse_pos.1 >= 0.0 && mouse_pos.1 <= screen_height() {
			context.set_mouse_cursor(CursorIcon::Text);
		}
		else {
			context.set_mouse_cursor(CursorIcon::Default);
		}
	}

	fn update_lines(&mut self) {
		let chars = self.buffer.to_vec();
		let mut start = 0;

		self.lines = Vec::new();

		for (i, chr) in chars.iter().enumerate() {
			if chr == &'\n' {
				self.lines.push(Line::new(start, i));
				start = i + 1;
			}
		}
	}
}
