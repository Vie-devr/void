use macroquad::prelude::*;

impl super::Editor {
	pub(super) fn execute_command(&mut self, key: KeyCode) {
		match key {
			// Ctrl Actions
			// -----------------------------
			// Move to the end of the nearest word at right
			KeyCode::Right if is_key_down(KeyCode::LeftControl) => {
				let mut move_amount = 0;
				let mut reached_word = false;

				for i in self.caret_pos..self.content.len() {
					let is_word_char = self.content[i].is_alphabetic() || (reached_word && self.content[i] == '_');

					if is_word_char {
						reached_word = true;
					}
					else if reached_word && !is_word_char {
						break;
					}

					move_amount += 1;
				}

				self.caret_pos += move_amount;
			},
			// Move to the start of the nearest word at left
			KeyCode::Left if is_key_down(KeyCode::LeftControl) => {
				let mut move_amount = 0;
				let mut reached_word = false;

				for i in (0..self.caret_pos).rev() {
					let is_word_char = self.content[i].is_alphabetic() || (reached_word && self.content[i] == '_');

					if is_word_char {
						reached_word = true;
					}
					else if reached_word && !is_word_char {
						break;
					}

					move_amount += 1;
				}

				self.caret_pos -= move_amount;
			},
			// Move to the start of document
			KeyCode::Up if is_key_down(KeyCode::LeftControl) => self.caret_pos = 0,
			// Move to the start of document
			KeyCode::Down if is_key_down(KeyCode::LeftControl) => self.caret_pos = self.content.len(),
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
				self.content.insert(self.caret_pos, '\t');
				self.caret_pos += 1;
			},
			// Print new line
			KeyCode::Enter => {
				self.content.insert(self.caret_pos, '\n');
				self.caret_pos += 1;
			},
			// Delete char before caret
			KeyCode::Backspace => {
				if self.caret_pos > 0 {
					self.content.remove(self.caret_pos - 1);
					self.caret_pos -= 1;
				}
			},
			// Delete char after caret
			KeyCode::Delete => {
				if self.caret_pos < self.content.len() {
					self.content.remove(self.caret_pos);
				}
			},
			// Move caret to the right by one char
			KeyCode::Right => {
				if self.caret_pos < self.content.len() {
					self.caret_pos += 1;
				}
			},
			// Move caret to the left by one char
			KeyCode::Left => {
				if self.caret_pos > 0 {
					self.caret_pos -= 1;
				}
			},
			// Print character
			_ => {
				let c = self.holding_char.unwrap();
				// TOneverDO: add hieroglyphs/emoji/other unicode symbols support
				if c.is_ascii() || c.is_alphabetic() {
					self.content.insert(self.caret_pos, c);
					self.caret_pos += 1;
				}
			},
		};
	}
}
