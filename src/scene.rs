use raylib::{RaylibHandle, RaylibThread, prelude::RaylibDrawHandle};

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Quit,
}

pub trait Scene {
    fn update(&mut self, rl: &RaylibHandle) -> Transition;
    fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread);
}
