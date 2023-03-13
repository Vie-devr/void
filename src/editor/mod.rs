mod style;
mod fs;

pub use style::*;

use macroquad::prelude::*;

const HOLDING_KEY_EXECUTION_START_DELAY: f32 = 0.4;
const HOLDING_KEY_EXECUTION_DELAY: f32 = 0.025;

pub struct Editor {
	content: Vec<char>,
	cursor_pos: usize,
	opened_file: Option<String>,
	style: EditorStyle,
	// Holding key stuff
	holding_key: Option<KeyCode>,
	holding_char: Option<char>,
	holding_timer: f32,
}

impl Editor {
	pub fn new(opened_file: Option<String>, style: EditorStyle) -> Self {
		let mut result = Self {
			content: Vec::new(),
			cursor_pos: 0,
			opened_file: None,
			style,
			holding_key: None,
			holding_char: None,
			holding_timer: 0.0,
		};

		// Open file if path is given
		if let Some(path) = opened_file {
			result.open_file_from_path(path);
		}

		result
	}

	pub fn update(&mut self) {
		if let Some(key) = get_last_key_pressed() {
			self.holding_key = Some(key);
			self.holding_char = Some(get_char_pressed().unwrap());

			self.execute_command(key);
		}

		// User hodling any key
		if let Some(key) = self.holding_key {
			// Pressed key was released
			if is_key_released(key) {
				self.holding_key = None;
				self.holding_timer = 0.0;
			}
			else {
				self.holding_timer += get_frame_time();
				
				// Checking if user holds key at least {HOLDING_KEY_EXECUTION_START_DELAY} seconds, and executing command with {HOLDING_KEY_EXECUTION_DELAY} delay
				if self.holding_timer >= HOLDING_KEY_EXECUTION_START_DELAY + HOLDING_KEY_EXECUTION_DELAY {
					self.execute_command(key);
					self.holding_timer = HOLDING_KEY_EXECUTION_START_DELAY;
				}
			}
		}
	}

	pub fn draw(&self, drawing_rect: Rect) {
		// Draw background
		draw_rectangle(
			drawing_rect.x,
			drawing_rect.y,
			drawing_rect.w,
			drawing_rect.h,
			self.style.background,
		);

		let text = self.content_as_text().replace("\t", "    ");

		// Draw editor content
		for (i, line) in text.lines().enumerate() {
			let y_coord = self.style.dimensions.height * (i + 1) as f32
					    + self.style.line_spacing * i as f32;

			draw_text_ex(
				line,
				self.style.text_padding,
				self.style.text_padding + y_coord,
				self.style.text_params,
			);
		}
	}

	fn execute_command(&mut self, key: KeyCode) {
		match key {
			// Ctrl Actions
			// -----------------------------
			// Move to the end of the nearest word at right
			KeyCode::Right if is_key_down(KeyCode::LeftControl) => {
				let mut move_amount = 0;
				let mut reached_non_whitespace = false;

				for i in self.cursor_pos..self.content.len() {
					if !self.content[i].is_whitespace() {
						reached_non_whitespace = true;
					}
					else if reached_non_whitespace && self.content[i].is_whitespace() {
						break;
					}

					move_amount += 1;
				}

				self.cursor_pos += move_amount;
			},
			// Move to the start of the nearest word at left
			KeyCode::Left if is_key_down(KeyCode::LeftControl) => {
				let mut move_amount = 0;
				let mut reached_non_whitespace = false;

				for i in (0..self.cursor_pos).rev() {
					if !self.content[i].is_whitespace() {
						reached_non_whitespace = true;
					}
					else if reached_non_whitespace && self.content[i].is_whitespace() {
						break;
					}

					move_amount += 1;
				}

				self.cursor_pos -= move_amount;
			},
			// Move to the start of document
			KeyCode::Up if is_key_down(KeyCode::LeftControl) => self.cursor_pos = 0,
			// Move to the start of document
			KeyCode::Down if is_key_down(KeyCode::LeftControl) => self.cursor_pos = self.content.len() - 1,
			// New file
			KeyCode::N if is_key_down(KeyCode::LeftControl) => self.new_file(),
			// Open file
			KeyCode::O if is_key_down(KeyCode::LeftControl) => self.open_file(),
			// Save file
			KeyCode::S if is_key_down(KeyCode::LeftControl) => self.save_file(),
			// Save file as
			KeyCode::S if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) => self.save_file_as(),
			// Other Actions
			// -----------------------------
			// Indent
			KeyCode::Tab => {
				self.content.insert(self.cursor_pos, '\t');
				self.cursor_pos += 1;
			},
			// Print new line
			KeyCode::Enter => {
				self.content.insert(self.cursor_pos, '\n');
				self.cursor_pos += 1;
			},
			// Delete char before caret
			KeyCode::Backspace => {
				if self.cursor_pos > 0 {
					self.content.remove(self.cursor_pos - 1);
					self.cursor_pos -= 1;
				}
			},
			// Delete char after caret
			KeyCode::Delete => {
				if self.cursor_pos + 1 < self.content.len() {
					self.content.remove(self.cursor_pos);
				}
			},
			// Move caret to the right by one char
			KeyCode::Right => {
				if self.cursor_pos + 1 < self.content.len() {
					self.cursor_pos += 1;
				}
			},
			// Move caret to the left by one char
			KeyCode::Left => {
				if self.cursor_pos > 0 {
					self.cursor_pos -= 1;
				}
			},
			// Print character
			_ => {
				// TOneverDO: add hieroglyphs/emoji/other unicode symbols support
				let c = self.holding_char.unwrap();
				if c.is_ascii() || c.is_alphabetic() {
					self.content.insert(self.cursor_pos, c);
					self.cursor_pos += 1;
				}
			},
		};
	}

	fn content_as_text(&self) -> String {
		self.content.iter().collect()
	}
}
