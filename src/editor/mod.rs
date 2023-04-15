mod gap_buffer;
mod text_drawer;

use crate::config::Config;
use clipboard::{ClipboardContext, ClipboardProvider};
use gap_buffer::GapBuffer;
use macroquad::{miniquad::CursorIcon, prelude::*};
use text_drawer::TextDrawer;

const FONT_REGULAR: &[u8] =
	include_bytes!("../../res/fonts/jet_brains_mono_regular.ttf");
const FONT_BOLD: &[u8] =
	include_bytes!("../../res/fonts/jet_brains_mono_extra_bold.ttf");

pub struct Editor {
	buffer: GapBuffer,
	caret_pos: usize,
	drawer: TextDrawer,
}

impl Editor {
	pub fn new() -> Self {
		let mut drawer = TextDrawer::new();
		drawer.add_font("regular", load_ttf_font_from_bytes(FONT_REGULAR).unwrap());
		drawer.add_font("bold", load_ttf_font_from_bytes(FONT_BOLD).unwrap());

		Self {
			buffer: GapBuffer::new(),
			caret_pos: 0,
			drawer,
		}
	}

	pub fn process_input(&mut self, key: KeyCode, chr: Option<char>) {
		let ctrl_pressed = is_key_down(KeyCode::LeftControl);

		self.ensure_ending_newline();

		match key {
			// Paste text from clipboard
			KeyCode::V if ctrl_pressed => {
				let mut ctx = ClipboardContext::new().unwrap();

				if let Ok(content) = ctx.get_contents() {
					let len = content.len();

					self.buffer.insert(&content, self.caret_pos);
					self.caret_pos += len;
				}
			}
			// Move caret right
			KeyCode::Right => {
				if self.caret_pos < self.buffer.len() {
					self.caret_pos += 1;
				}
			}
			// Move caret left
			KeyCode::Left => {
				if self.caret_pos > 0 {
					self.caret_pos -= 1;
				}
			}
			// Print new line
			KeyCode::Enter => {
				self.buffer.insert_char('\n', self.caret_pos);
				self.caret_pos += 1;
			}
			// Print tabulation
			KeyCode::Tab => {
				self.buffer.insert_char('\t', self.caret_pos);
				self.caret_pos += 1;
			}
			// Delete char before caret
			KeyCode::Backspace => {
				if self.caret_pos > 0 {
					self.buffer.delete_char(self.caret_pos - 1);
					self.caret_pos -= 1;
				}
			}
			// Delete char after caret
			KeyCode::Delete => {
				if self.caret_pos < self.buffer.len() {
					self.buffer.delete_char(self.caret_pos);
				}
			}
			// Print char
			_ => {
				if let Some(chr) = chr {
					if chr.is_ascii() || chr.is_alphabetic() {
						self.buffer.insert_char(chr, self.caret_pos);
						self.caret_pos += 1;
					}
				}
			}
		}
	}

	pub fn draw(&self, config: &Config) {
		clear_background(config.get_color("background0"));

		let text = self
			.buffer
			.to_string()
			.replace('\t', &" ".repeat(config.tab_size()));
		let lines = text.lines();
		let line_nums_width = self.drawer.measure_text(
			&format!(" {} ", lines.clone().count() + 1),
			config.text_size() as u16,
			"regular",
		).width;

		draw_rectangle(
			0.0,
			0.0,
			line_nums_width,
			screen_height(),
			config.get_color("background1"),
		);

		for (i, line) in lines.clone().enumerate() {
			self.draw_line_num(config, i);

			self.drawer.draw_text(
				line,
				line_nums_width,
				(config.text_size() * i) as f32,
				config.text_size() as u16,
				config.get_color("foreground0"),
				"regular",
			);
		}

		self.draw_line_num(config, lines.clone().count());

		Self::update_mouse_cursor();
	}

	fn ensure_ending_newline(&mut self) {
		if let Some(last) = self.buffer.to_vec().last() {
			if last == &'\n' {
				return;
			}
		}

		self.buffer.insert_char('\n', self.buffer.len());
	}

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

	fn draw_line_num(&self, config: &Config, line: usize) {
		let y = (config.text_size() * line) as f32;

		self.drawer.draw_text(
			&format!(" {} ", line + 1),
			0.0,
			y,
			config.text_size() as u16,
			config.get_color("foreground1"),
			"regular",
		);
	}
}
