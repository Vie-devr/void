use macroquad::prelude::*;
use crate::{
	editor::Editor,
	config::Config,
	utils::config_file,
};

const HOLDING_KEY_START_DELAY: f32 = 0.4;
const HOLDING_KEY_DELAY: f32 = 0.03;

pub struct App {
	editor: Editor,
	config: Config,
	key_holding_timer: f32,
	holding_key: Option<KeyCode>,
	holding_char: Option<char>,
}

impl App {
	pub fn new() -> Self {
		let config = Config::from_file(&config_file());

		if config.is_err() {
			println!("{}", config.as_ref().unwrap_err());
		}

		Self {
			editor: Editor::new(),
			config: config.unwrap_or_default(),
			key_holding_timer: 0.0,
			holding_key: None,
			holding_char: None,
		}
	}

	pub fn update(&mut self) {
		// Pressed key
		if let Some(key) = get_last_key_pressed() {
			self.key_holding_timer = 0.0;
			self.holding_key = Some(key);
			self.holding_char = get_char_pressed();

			self.process_input();
		}

		// Holding key
		if let Some(key) = self.holding_key {
			// Still holding key
			if is_key_down(key) {
		    	// Update timer
				self.key_holding_timer += get_frame_time();

				// Wait start delay and delay
				if self.key_holding_timer >= HOLDING_KEY_START_DELAY + HOLDING_KEY_DELAY {
					self.key_holding_timer = HOLDING_KEY_START_DELAY;
					self.process_input();
				}
			}
			else {
				self.holding_key = None;
			}
		}
	}

	pub fn draw(&self) {
		self.editor.draw(&self.config);
	}

	fn process_input(&mut self) {
		let key = self.holding_key.unwrap();

		match key {
			_ => self.editor.process_input(key, self.holding_char),
		}
	}
}
