mod style;

pub use style::*;

use macroquad::prelude::*;

pub struct Editor {
	pub pre_caret: Vec<char>,
	pub selection: Vec<char>,
	pub post_caret: Vec<char>,
	pub style: EditorStyle,
}

impl Editor {
	pub fn new(style: EditorStyle) -> Self {
		Self {
			pre_caret: Vec::new(),
			selection: Vec::new(),
			post_caret: Vec::new(),
			style,
		}
	}

	pub fn update(&mut self) {
		if let Some(c) = get_char_pressed() {
			// TODO: add hieroglyphs/emoji/other unicode symbols support
			if c.is_ascii() || c.is_alphabetic() {
				self.pre_caret.push(c);
			}
			else if let Some(key) = get_last_key_pressed() {
				self.execute_command(key);
			}
		}
	}

	pub fn draw(&self) {
		// Draw background
		draw_rectangle(
			0.,
			0.,
			screen_width(),
			screen_height(),
			self.style.background,
		);

		// Concat vectors
		let mut full_text: Vec<char> = Vec::new();
		full_text.extend(&self.pre_caret);
		full_text.extend(&self.selection);
		// Reverse post_caret, because we are storing it in reversed order, for convenience
		full_text.extend(self.post_caret.iter().rev().collect::<Vec<_>>());

		// And convert them to string
		let mut full_text: String = full_text.iter().collect();
		full_text = full_text.replace("\t", "    ");

		let dimensions = self.style.measure_text(&full_text);

		// Draw editor content
		for (i, line) in full_text.lines().enumerate() {
			let y_coord = dimensions.height + (dimensions.height + self.style.line_spacing) * i as f32;

			draw_text_ex(
				line,
				self.style.text_padding,
				self.style.text_padding + y_coord,
				self.style.text_params,
			);
		}
	}

	pub fn execute_command(&mut self, key: KeyCode) {
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
			_ => {},
		};
	}
}
