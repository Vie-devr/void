mod style;
mod fs;
mod command_executor;
mod drawing;

pub use style::*;

use macroquad::prelude::*;

const HOLDING_KEY_EXECUTION_START_DELAY: f32 = 0.4;
const HOLDING_KEY_EXECUTION_DELAY: f32 = 0.025;

struct Line {
	start: usize,
	end: usize,
}

impl Line {
	fn new(start: usize, end: usize) -> Self {
		Self {
			start,
			end,
		}
	}
}

pub struct Editor {
	content: Vec<char>,
	lines: Vec<Line>,
	caret_pos: usize,
	opened_file: Option<String>,
	style: EditorStyle,
	// Holding keys stuff
	holding_key: Option<KeyCode>,
	holding_char: Option<char>,
	holding_timer: f32,
}

impl Editor {
	pub fn new(opened_file: Option<String>, style: EditorStyle) -> Self {
		let mut result = Self {
			content: vec!['\n'],
			lines: vec![Line::new(0, 0)],
			caret_pos: 0,
			opened_file: None,
			style,
			holding_key: None,
			holding_char: None,
			holding_timer: 0.0,
		};

		if let Some(path) = opened_file {
			result.open_file_from_path(path);
		}

		result
	}

	pub fn update(&mut self) {
		if let Some(key) = get_last_key_pressed() {
			self.holding_key = Some(key);
			self.holding_timer = 0.0;

			if let Some(c) = get_char_pressed() {
				self.holding_char = Some(c);
			}

			self.execute_command(key);
		}

		if let Some(key) = self.holding_key {
			if is_key_released(key) {
				self.holding_key = None;
			}
			else {
				self.holding_timer += get_frame_time();
				
				if self.holding_timer >= HOLDING_KEY_EXECUTION_START_DELAY + HOLDING_KEY_EXECUTION_DELAY {
					self.holding_timer = HOLDING_KEY_EXECUTION_START_DELAY;
	
					self.execute_command(key);
				}
			}
		}
	}

	fn caret_row(&self) -> usize {
		for (i, line) in self.lines.iter().enumerate() {
			if self.caret_pos >= line.start && self.caret_pos <= line.end {
				return i;
			}
		}

		0
	}

	fn caret_col(&self) -> usize {
		let new_line = &self.lines[self.caret_row()];
		let col = self.caret_pos - new_line.start;
		let tabs = self.content[new_line.start..self.caret_pos]
					.iter()
					.filter(|c| **c == '\t')
					.count();

		// We need perceive \t as n spaces for correct caret working
		col + tabs * (self.style.tab_size - 1)
	}

	fn update_lines(&mut self) {
		let mut start = 0;

		self.lines = Vec::new();

		for (i, c) in self.content.iter().enumerate() {
			if c == &'\n' {
				self.lines.push(Line::new(start, i));
				start = i + 1;
			}
		}
	}
}
