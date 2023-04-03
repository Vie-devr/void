mod editor;
mod app;

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
