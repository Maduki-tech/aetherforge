mod util;

use raylib::prelude::*;

use crate::util::is_clickable_rect;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Aetherforge")
        .build();

    // gameloop
    while !rl.window_should_close() {
        let pos = Vector2::new(50.0, 100.0);
        let size = Vector2::new(30.0, 30.0);

        if is_clickable_rect(&rl, pos, size) {
            println!("Clicked the shit button");
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_rectangle_v(pos, size, Color::RED);

        d.draw_fps(10, 10);
    }
}
