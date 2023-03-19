mod style;
mod fs;
mod command_executor;

pub use style::*;

use macroquad::prelude::*;

const HOLDING_KEY_EXECUTION_START_DELAY: f32 = 0.4;
const HOLDING_KEY_EXECUTION_DELAY: f32 = 0.025;

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

struct Line {
	start: usize,
	end: usize,
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

		// Open file if path given
		if let Some(path) = opened_file {
			result.open_file_from_path(path);
		}

		result
	}

	pub fn update(&mut self) {
		// Pressed any key
		if let Some(key) = get_last_key_pressed() {
			// Reset key holding stuff
			self.holding_key = Some(key);
			self.holding_timer = 0.0;

			if let Some(c) = get_char_pressed() {
				self.holding_char = Some(c);
			}

			self.execute_command(key);
		}

		// User hodling any key
		if let Some(key) = self.holding_key {
			// Pressed key was released
			if is_key_released(key) {
				self.holding_key = None;
			}
			else {
				self.holding_timer += get_frame_time();
				
				// Checking if user holds key at least {HOLDING_KEY_EXECUTION_START_DELAY} seconds, and executing command with {HOLDING_KEY_EXECUTION_DELAY} delay
				if self.holding_timer >= HOLDING_KEY_EXECUTION_START_DELAY + HOLDING_KEY_EXECUTION_DELAY {
					// Reset timer
					self.holding_timer = HOLDING_KEY_EXECUTION_START_DELAY;
	
					self.execute_command(key);
				}
			}
		}
	}

	pub fn draw(&self, drawing_rect: Rect) {
		let line_nums_bar_width = self.style.measure_text(&format!(" {} ", self.lines.len())).width;

		// Draw background
		draw_rectangle(
			drawing_rect.x + line_nums_bar_width,
			drawing_rect.y,
			drawing_rect.w - line_nums_bar_width,
			drawing_rect.h,
			self.style.background,
		);

		// Draw line numbers bar
		draw_rectangle(
			drawing_rect.x,
			drawing_rect.y,
			line_nums_bar_width,
			drawing_rect.h,
			self.style.line_nums_background,
		);

		let text = self.content_as_text().replace("\t", &" ".repeat(self.style.tab_size));

		// Draw editor content
		for (i, line) in text.lines().enumerate() {
			let x = drawing_rect.x + self.style.text_padding + line_nums_bar_width;
			let y = drawing_rect.y + self.style.text_padding
				  + self.style.dimensions.height * (i + 1) as f32
				  + self.style.line_spacing * i as f32;

			// Draw line number
			draw_text_ex(
				// I am too lazy to make here real padding. just spaces.
				&format!(" {} ", i + 1),
				drawing_rect.x,
				y,
				self.style.line_nums_params,
			);

			// Draw line
			draw_text_ex(
				line,
				x,
				y,
				self.style.text_params,
			);

			if self.caret_row() == i {
				let x = x + self.style.dimensions.width * self.caret_col() as f32;
				let c = self.content[self.caret_pos];

				// Draw caret
				draw_rectangle(
					x,
					y - self.style.dimensions.height,
					self.style.dimensions.width,
					self.style.dimensions.height,
					self.style.text,
				);

				if !c.is_whitespace() {
					let mut params = self.style.text_params;
					params.color = self.style.background;

					// Draw char over the caret
					draw_text_ex(
						&c.to_string(),
						x,
						y,
						params,
					);
				}
			}
		}
	}
	
	fn content_as_text(&self) -> String {
		String::from_iter(self.content.iter())
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
		// Because we are rendering tab as n spaces, we need caret to move like there is n spaces
		let tabs = self.content[new_line.start..self.caret_pos]
					.iter()
					.filter(|c| **c == '\t')
					.count();

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

impl Line {
	fn new(start: usize, end: usize) -> Self {
		Self {
			start,
			end,
		}
	}
}
