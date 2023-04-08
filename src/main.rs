mod app;
mod config;
mod editor;
mod theme;
mod utils;

use app::App;
use macroquad::prelude::*;

#[macroquad::main("Void")]
async fn main() {
	let mut app = App::new();

	loop {
		app.update();
		app.draw();

		next_frame().await
	}
}
