use macroquad::prelude::*;

impl super::Editor {
	pub(super) fn content_as_text(&self) -> String {
		String::from_iter(self.content.iter())
	}

	pub(super) fn caret_row(&self) -> usize {
		for (i, line) in self.lines.iter().enumerate() {
			if self.caret_pos >= line.start && self.caret_pos <= line.end {
				return i;
			}
		}

		0
	}

	pub(super) fn caret_col(&self) -> usize {
		let line = &self.lines[self.caret_row()];
		let col = self.caret_pos - line.start;
		// Because we are rendering tab as n spaces, we need caret to move like there is n spaces
		let tabs = self.content[line.start..self.caret_pos]
						.iter()
						.filter(|&c| c == &'\t')
						.count();

		col + tabs * (self.style.tabs - 1)
	}

	pub(super) fn caret_screen_pos(&self) -> (f32, f32) {
		let mut pos = (
			self.style.text_padding,
			self.style.text_padding,
		);

		let (col, row) = (self.caret_col() as f32, self.caret_row() as f32);

		pos.0 += col * self.style.dimensions.width;
		pos.1 += row * self.style.dimensions.height;
		pos.1 += row * self.style.line_spacing;

		pos
	}

	pub(super) fn update_lines(&mut self) {
		let mut begin = 0;

		self.lines = Vec::new();

		// We need at least one line in the document
		if self.content.is_empty() || self.content.last().unwrap() != &'\n' {
			self.content.push('\n');
		}

		for (i, c) in self.content.iter().enumerate() {
			if c == &'\n' {
				self.lines.push(super::Line::new(begin, i));
				begin = i + 1;
			}
		}
	}
}
