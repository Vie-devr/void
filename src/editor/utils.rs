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
		self.caret_pos - self.lines[self.caret_row()].start
	}

	pub(super) fn caret_screen_pos(&self) -> (f32, f32) {
		let mut pos = (
			self.style.text_padding,
			self.style.text_padding,
		);

		pos.0 += self.caret_col() as f32 * self.style.dimensions.width;
		pos.1 += self.caret_row() as f32 * self.style.dimensions.height
			   + self.caret_row() as f32 * self.style.line_spacing;

		pos
	}

	pub(super) fn update_lines(&mut self) {
		let mut begin = 0;

		self.lines = Vec::new();

		for (i, c) in self.content.iter().enumerate() {
			if c == &'\n' {
				self.lines.push(super::Line::new(begin, i));
				begin = i + 1;
			}
		}
	}
}
