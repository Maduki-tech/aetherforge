use crate::{
    button::Button,
    scene::{Scene, Transition},
};
use raylib::prelude::*;

#[derive(Debug)]
pub struct GameMenu {
    back_button: Button,
}

impl GameMenu {
    pub fn new(_rl: &RaylibHandle) -> Self {
        GameMenu {
            back_button: Button::new(10, 10, 100, 50, Color::BLUE, "Back".to_string()),
        }
    }
}

impl Scene for GameMenu {
    fn update(&mut self, rl: &RaylibHandle) -> Transition {
        if self.back_button.is_clicked(rl) {
            return Transition::Pop;
        }
        Transition::None
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.draw_text("Game Menu", 10, 10, 20, Color::WHITE);
        self.back_button.draw(d);
    }
}
