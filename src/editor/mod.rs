mod gap_buffer;
mod text_drawer;

use macroquad::{
	prelude::*,
	miniquad::CursorIcon,
};
use copypasta::{ClipboardContext, ClipboardProvider};
use gap_buffer::GapBuffer;
use text_drawer::TextDrawer;
use crate::config::Config;

const FONT: &[u8] = include_bytes!("../../res/jet_brains_mono.ttf");

pub struct Editor {
	buffer: GapBuffer,
	caret_pos: usize,
	drawer: TextDrawer,
}

impl Editor {
	pub fn new() -> Self {
		Self {
			buffer: GapBuffer::new(),
			caret_pos: 0,
			drawer: TextDrawer::new(
				load_ttf_font_from_bytes(FONT).unwrap(),
			),
		}
	}

	pub fn process_input(&mut self, key: KeyCode, chr: Option<char>) {
		let ctrl_pressed = is_key_down(KeyCode::LeftControl);

		self.ensure_ending_newline();

		match key {
			// Paste text from clipboard (the most editor's weight is copypasta crate, lol)
			KeyCode::V if ctrl_pressed => {
				let mut ctx = ClipboardContext::new().unwrap();

				if let Ok(content) = ctx.get_contents() {
					let len = content.len();

					self.buffer.insert(content, self.caret_pos);
					self.caret_pos += len;
				}
			},
			// Move caret right
			KeyCode::Right => {
				if self.caret_pos < self.buffer.len() {
					self.caret_pos += 1;
				}
			},
			// Move caret left
			KeyCode::Left => {
				if self.caret_pos > 0 {
					self.caret_pos -= 1;
				}
			},
			// Print new line
			KeyCode::Enter => {
				self.buffer.insert_char('\n', self.caret_pos);
				self.caret_pos += 1;
			},
			// Print tabulation
			KeyCode::Tab => {
				self.buffer.insert_char('\t', self.caret_pos);
				self.caret_pos += 1;
			},
			// Delete char before caret
			KeyCode::Backspace => {
				if self.caret_pos > 0 {
					self.buffer.delete_char(self.caret_pos - 1);
					self.caret_pos -= 1;
				}
			},
			// Delete char after caret
			KeyCode::Delete => {
				if self.caret_pos < self.buffer.len() {
					self.buffer.delete_char(self.caret_pos);
				}
			},
			// Print char
			_ => {
				if let Some(chr) = chr {
					if chr.is_ascii() || chr.is_alphabetic() {
						self.buffer.insert_char(chr, self.caret_pos);
						self.caret_pos += 1;
					}
				}
			},
		}
	}

	pub fn draw(&self, config: &Config) {
		clear_background(config.theme.background);

		let text = self.buffer.to_string()
						.replace('\t', &" ".repeat(config.tab_size));
		let lines = text.lines();

		for (i, line) in lines.enumerate() {
			self.drawer.draw_text(
				line,
				0.0,
				(config.text_size as usize * i) as f32,
				config.text_size,
				config.theme.foreground,
			);
		}

		self.update_mouse_cursor();
	}

	fn ensure_ending_newline(&mut self) {
		if let Some(last) = self.buffer.to_vec().last() {
			if last == &'\n' {
				return;
			}
		}

		self.buffer.insert_char('\n', self.buffer.len());
	}

	fn update_mouse_cursor(&self) {
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
}
