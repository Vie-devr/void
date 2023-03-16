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
			// Indent
			KeyCode::Tab => {
				self.content.insert(self.caret_pos, '\t');
				self.caret_pos += 1;
			},
			// Print new line
			KeyCode::Enter => {
				self.content.insert(self.caret_pos, '\n');
				self.caret_pos += 1;
				self.update_lines();
			},
			// Delete char before caret
			KeyCode::Backspace => self.delete_char(self.caret_pos as i32 - 1),
			// Delete char after caret
			KeyCode::Delete => self.delete_char(self.caret_pos as i32),
			// Move one line up
			KeyCode::Up => self.move_to_line(self.caret_row() as i32 - 1),
			// Move one line down
			KeyCode::Down => self.move_to_line(self.caret_row() as i32 + 1),
			// Move caret to the right by one char
			KeyCode::Right => {
			    // Caret is not at the end of document
			    if self.caret_pos < self.content.len() {
			        self.caret_pos += 1;
			    }
			},
			// Move caret to the left by one char
			KeyCode::Left => {
				// Caret is not at the start of line
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
					self.update_lines();
				}
			},
		};
	}

	fn delete_char(&mut self, i: i32) {
		// Character we want to delete is in content bounds
		if i >= 0 && (i as usize) < self.content.len() {
			let i = i as usize;

			self.content.remove(i);

			// We need at least one line in the document
			if self.content.is_empty() || self.content.last().unwrap() != &'\n' {
				self.content.push('\n');
			}

			self.update_lines();

			if i < self.caret_pos {
				self.caret_pos -= 1;
			}
		}
	}

	fn move_to_line(&mut self, i: i32) {
		// Line exists
		if i >= 0 && (i as usize) < self.lines.len() {
			let new_line = &self.lines[i as usize];

			// Move to the new line
			self.caret_pos = new_line.start + usize::min(self.caret_col(), new_line.end - new_line.start);
		}
		else if i < 0 {
			self.caret_pos = 0;
		}
		else {
			self.caret_pos = self.content.len() - 1;
		}
	}
}
