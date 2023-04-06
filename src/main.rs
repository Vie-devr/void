mod editor;
mod app;
mod config;
mod theme;
mod utils;

use macroquad::prelude::*;
use app::App;

#[macroquad::main("Void")]
async fn main() {
	let mut app = App::new();

    loop {
    	app.update();
		app.draw();

		next_frame().await
    }
}
