mod app;
mod config;
mod themes;
mod editor;
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
