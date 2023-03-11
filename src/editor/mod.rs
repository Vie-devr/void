mod style;
mod fs;

pub use style::*;

use macroquad::prelude::*;

pub struct Editor {
	pre_caret: Vec<char>,
	selection: Vec<char>,
	post_caret: Vec<char>,
	opened_file: Option<String>,
	style: EditorStyle,
}

impl Editor {
	pub fn new(opened_file: Option<String>, style: EditorStyle) -> Self {
		let mut result = Self {
			pre_caret: Vec::new(),
			selection: Vec::new(),
			post_caret: Vec::new(),
			opened_file: None,
			style,
		};

		// Open file if path is given
		if let Some(path) = opened_file {
			result.open_file_from_path(path);
		}

		result
	}

	pub fn update(&mut self) {
		if let Some(c) = get_char_pressed() {
			// TODO: add hieroglyphs/emoji/other unicode symbols support
			// If Ctrl pressed, we want to execute command
			if (c.is_ascii() || c.is_alphabetic()) && !is_key_down(KeyCode::LeftControl) {
				self.pre_caret.push(c);
			}
			else if let Some(key) = get_last_key_pressed() {
				self.execute_command(key);
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

		// Concat vectors
		let text = self.get_text().replace("\t", "    ");

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
			// Indent
			KeyCode::Tab => self.pre_caret.push('\t'),
			// Print new line
			KeyCode::Enter => self.pre_caret.push('\n'),
			// Delete char before caret
			KeyCode::Backspace => { self.pre_caret.pop(); },
			// Delete char after caret
			KeyCode::Delete => { self.post_caret.pop(); },
			// Move caret to the left by one char
			KeyCode::Left => {
				if let Some(element) = self.pre_caret.pop() {
					self.post_caret.push(element);
				}
			},
			// Move caret to the right by one char
			KeyCode::Right => {
				if let Some(element) = self.post_caret.pop() {
					self.pre_caret.push(element);
				}
			},
			// Move to the start of document
			KeyCode::Up if is_key_down(KeyCode::LeftControl) => {
				self.post_caret.extend(self.pre_caret.iter().rev().collect::<Vec<_>>());
				self.pre_caret.clear();
			},
			// Move to the start of document
			KeyCode::Down if is_key_down(KeyCode::LeftControl) => {
				self.pre_caret.extend(self.post_caret.iter().rev().collect::<Vec<_>>());
				self.post_caret.clear();
			},
			// New file
			KeyCode::N if is_key_down(KeyCode::LeftControl) => self.new_file(),
			// Open file
			KeyCode::O if is_key_down(KeyCode::LeftControl) => self.open_file(),
			// Save file as
			KeyCode::S if is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::LeftShift) => self.save_file_as(),
			// Save file
			KeyCode::S if is_key_down(KeyCode::LeftControl) => self.save_file(),
			_ => {},
		};
	}

	fn get_text(&self) -> String {
		// Concat vectors
		let mut text: Vec<char> = Vec::new();
		text.extend(&self.pre_caret);
		text.extend(&self.selection);
		// Reverse post_caret, because we are storing it in reversed order, for convenience
		text.extend(self.post_caret.iter().rev().collect::<Vec<_>>());

		// And convert them to string
		text.iter().collect()
	}
}
