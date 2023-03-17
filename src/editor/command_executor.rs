use macroquad::prelude::*;

impl super::Editor {
	pub(super) fn execute_command(&mut self, key: KeyCode) {
		match key {
			// Ctrl Actions
			// -----------------------------
			// Move to the start of document
			KeyCode::Up if is_key_down(KeyCode::LeftControl) => self.caret_pos = 0,
			// Move to the start of document
			KeyCode::Down if is_key_down(KeyCode::LeftControl) => self.caret_pos = self.content.len() - 1,
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
			// Delete tabulation at the start of line
			KeyCode::Tab if is_key_down(KeyCode::LeftShift) => {
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
				// Caret is not at the end of document
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
			// Move caret to the right by one char
			KeyCode::Right => {
				// Caret is not at the end of document
				if self.caret_pos < self.content.len() - 1 {
					self.caret_pos += 1;
				}
			},
			// Move caret to the left by one char
			KeyCode::Left => {
				// Caret is not at the start of document
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
		
		self.caret_pos = new_line.start + usize::min(self.caret_col(), new_line.end - new_line.start);
	}
}
