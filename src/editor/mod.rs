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
	/// caret_pos.0 - line which caret is on, caret_pos.1 - caret position in this line
	caret_pos: (usize, usize),
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
			lines: vec![Line::new(0, 1)],
			caret_pos: (0, 0),
			opened_file: None,
			style,
			holding_key: None,
			holding_char: None,
			holding_timer: 0.0,
		};

		// Open file if path is given
		if let Some(path) = opened_file {
			result.open_file_from_path(path);
		}

		result
	}

	pub fn update(&mut self) {
		if let Some(key) = get_last_key_pressed() {
			self.holding_key = Some(key);
			self.holding_char = Some(get_char_pressed().unwrap());
			self.holding_timer = 0.0;

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
					self.execute_command(key);
					self.holding_timer = HOLDING_KEY_EXECUTION_START_DELAY;
				}
			}
		}

		// Content is not empty and not ends with a new line
		if self.content.is_empty() || self.content.last().unwrap() != &'\n' {
			self.content.push('\n');
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

		let text = self.content_as_text().replace("\t", "    ");

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

		let caret_screen_pos = self.caret_screen_position();

		draw_rectangle(
			caret_screen_pos.0,
			caret_screen_pos.1,
			self.style.caret_width,
			self.style.dimensions.height,
			self.style.caret,
		);
	}

	fn content_as_text(&self) -> String {
		String::from_iter(self.content.iter())
	}

	fn caret_abs_pos(&self) -> usize {
		self.lines[self.caret_pos.0].start + self.caret_pos.1
	}

	fn caret_screen_position(&self) -> (f32, f32) {
		let mut pos = (
			self.style.text_padding,
			self.style.text_padding,
		);

		pos.0 += self.caret_pos.1 as f32 * self.style.dimensions.width;
		pos.1 += self.caret_pos.0 as f32 * self.style.dimensions.height
			   + self.caret_pos.0 as f32 * self.style.line_spacing;

		pos
	}

	pub fn update_lines(&mut self) {
		self.lines = Vec::new();

		let mut begin = 0;

		for (i, c) in self.content.iter().enumerate() {
			if c == &'\n' {
				self.lines.push(Line::new(begin, i));
				begin = i + 1;
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
