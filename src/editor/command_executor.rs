use macroquad::prelude::*;
use crate::utils::alphanumeric;

impl super::Editor {
	pub(super) fn execute_command(&mut self, key: KeyCode) {
		let ctrl = is_key_down(KeyCode::LeftControl);
		let shift = is_key_down(KeyCode::LeftShift);
		let alt = is_key_down(KeyCode::LeftAlt);

		match key {
			// Ctrl Actions
			// -----------------------------
			// Delete word before caret
			KeyCode::Backspace if ctrl => self.move_one_word_left(true),
			// Delete word after caret
			KeyCode::Delete if ctrl => self.move_one_word_right(true),
			// Move to the start of document
			KeyCode::Up if ctrl => self.caret_pos = 0,
			// Move to the start of document
			KeyCode::Down if ctrl => self.caret_pos = self.content.len() - 1,
			// Move caret to the right by one word
			KeyCode::Right if ctrl => self.move_one_word_right(false),
			// Move caret to the left by one word
			KeyCode::Left if ctrl => self.move_one_word_left(false),
			// New file
			KeyCode::N if ctrl => self.new_file(),
			// Open file
			KeyCode::O if ctrl => self.open_file(),
			// Save file as
			KeyCode::S if ctrl && shift => self.save_file_as(),
			// Save file
			KeyCode::S if ctrl => self.save_file(),
			// Other Actions
			// -----------------------------
			// Delete tabulation at the start of line
			KeyCode::Tab if shift => {
				let line_start = self.lines[self.caret_row()].start;

				if self.content[line_start] == '\t' {
					self.content.remove(line_start);
					self.caret_pos -= 1;

					self.update_lines();
				}
			},
			// Insert tabulation at the start of line
			KeyCode::Tab => {
				self.content.insert(self.lines[self.caret_row()].start, '\t');
				self.caret_pos += 1;

				self.update_lines();
			},
			// Print new line
			KeyCode::Enter => {
				self.content.insert(self.caret_pos, '\n');
				self.caret_pos += 1;
				self.update_lines();
			},
			// Delete char before caret
			KeyCode::Backspace => {
				if self.caret_pos > 0 {
					self.caret_pos -= 1;
					self.content.remove(self.caret_pos);

					self.update_lines();
				}
			},
			// Delete char after caret
			KeyCode::Delete => {
				if self.caret_pos < self.content.len() - 1 {
					self.content.remove(self.caret_pos);

					self.update_lines();
				}
			},
			// Move one line up
			KeyCode::Up => {
				let row = self.caret_row();

				if row == 0 {
					self.caret_pos = 0;
					return;
				}

				self.move_to_line(row - 1);
			}
			// Move one line down
			KeyCode::Down => {
				let row = self.caret_row();

				if row == self.lines.len() - 1 {
					self.caret_pos = self.content.len() - 1;
					return;
				}

				self.move_to_line(self.caret_row() + 1);
			},
			// Move caret to the end of line
			KeyCode::Right if alt => {
				self.caret_pos = self.lines[self.caret_row()].end;
			},
			// Move caret to the start of line
			KeyCode::Left if alt => {
				self.caret_pos = self.lines[self.caret_row()].start;
			},
			// Move caret to the right by one char
			KeyCode::Right => {
				if self.caret_pos < self.content.len() - 1 {
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
				if let Some(c) = self.holding_char {
					// TOneverDO: add hieroglyphs/emoji/other unicode symbols support
					if c.is_ascii() || c.is_alphabetic() {
						self.content.insert(self.caret_pos, c);
						self.caret_pos += 1;
						self.update_lines();
					}
				}
			},
		};
	}

	fn move_to_line(&mut self, i: usize) {
		let new_line = &self.lines[i];
		
		self.caret_pos = new_line.start + self.caret_col().clamp(0, new_line.end - new_line.start);
	}

	fn move_one_word_right(&mut self, delete_word: bool) {
		while self.caret_pos < self.content.len() - 1 && !alphanumeric(self.content[self.caret_pos]) {
			self.caret_pos += 1;

			if delete_word {
				self.content.remove(self.caret_pos);
			}
		}

		while self.caret_pos < self.content.len() - 1 && alphanumeric(self.content[self.caret_pos]) {
			self.caret_pos += 1;

			if delete_word {
				self.content.remove(self.caret_pos);
			}
		}

		self.update_lines();
	}
	fn move_one_word_left(&mut self, delete_word: bool) {

		while self.caret_pos > 0 && !alphanumeric(self.content[self.caret_pos - 1]) {
			self.caret_pos -= 1;

			if delete_word {
				self.content.remove(self.caret_pos);
			}
		}

		while self.caret_pos > 0 && alphanumeric(self.content[self.caret_pos - 1]) {
			self.caret_pos -= 1;

			if delete_word {
				self.content.remove(self.caret_pos);
			}
		}

		self.update_lines();
	}
}
