mod editor;

use macroquad::prelude::*;
use std::env::args;

#[macroquad::main("Void")]
async fn main() {
	let mut editor = editor::Editor::new(
		args().nth(1),
		editor::EditorStyle::new(
			4,
			7.5,
			4.8,
			2.0,
			load_ttf_font_from_bytes(include_bytes!("../res/jet_brains_mono.ttf")).unwrap(),
			80,
			0.25,
			Color::from_rgba(45, 45, 45, 255),
			Color::from_rgba(201, 209, 217, 255),
			Color::from_rgba(201, 209, 217, 255),
			Color::from_rgba(55, 55, 55, 255),
			Color::from_rgba(115, 115, 115, 255),
		),
	);

	loop {
		editor.update();
		editor.draw(Rect::new(0., 0., screen_width(), screen_height()));

		next_frame().await
	}
}
