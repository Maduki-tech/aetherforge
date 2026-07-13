mod button;
mod logger;
mod scene;
mod scenes;
mod util;

use raylib::prelude::*;
use scene::{Scene, Transition};
use scenes::main_menu::MainMenu;

fn main() {
    logger::init();
    let (mut rl, thread) = raylib::init().fullscreen().title("Aetherforge").build();
    let mut current_scene: Box<dyn Scene> = Box::new(MainMenu::new(&mut rl, &thread));

    while !rl.window_should_close() {
        match current_scene.update(&rl) {
            Transition::Push(next) => current_scene = next,
            Transition::Quit => break,
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        current_scene.draw(&mut d, &thread);
    }
}
