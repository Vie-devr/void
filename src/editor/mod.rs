mod gap_buffer;

use macroquad::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use gap_buffer::GapBuffer;

const TEXT_SIZE: f32 = 48.0;
const TEXT_COLOR: Color = WHITE;
const TAB_SIZE: usize = 4;

pub struct Editor {
	buffer: GapBuffer,
	caret_pos: usize,
}

impl Editor {
	pub fn new() -> Self {
		Self {
			buffer: GapBuffer::new(),
			caret_pos: 0,
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

	pub fn draw(&self) {
		let text = self.buffer.to_string().replace('\t', &" ".repeat(TAB_SIZE));

		for (i, line) in text.lines().enumerate() {
			draw_text(
				line,
				0.0,
				TEXT_SIZE * (i + 1) as f32,
				TEXT_SIZE,
				TEXT_COLOR,
			);
		}
	}

	fn ensure_ending_newline(&mut self) {
		if let Some(last) = self.buffer.to_vec().last() {
			if last == &'\n' {
				return;
			}
		}

		self.buffer.insert_char('\n', self.buffer.len());
	}
}
