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
    let mut current_scene: Vec<Box<dyn Scene>> = Vec::new();
    current_scene.push(Box::new(MainMenu::new(&mut rl, &thread)));

    while !rl.window_should_close() {
        match current_scene.last_mut().unwrap().update(&rl) {
            Transition::Push(next) => current_scene.push(next),
            Transition::Pop => {
                current_scene.pop();
                if current_scene.is_empty() {
                    break;
                }
            }
            Transition::Quit => break,
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        current_scene.last_mut().unwrap().draw(&mut d, &thread);
    }
}
