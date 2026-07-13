use log::info;
use raylib::{prelude::*, texture::Texture2D};

use crate::{
    button::Button,
    scene::{Scene, Transition},
};

const BTN_W: i32 = 200;
const BTN_H: i32 = 50;

pub struct MainMenu {
    background: Texture2D,
    play_button: Button,
    quit_button: Button,
}

impl MainMenu {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let screen_w = rl.get_screen_width();
        let screen_h = rl.get_screen_height();

        MainMenu {
            background: rl.load_texture(thread, "assets/image.png").unwrap(),
            play_button: Button::new(
                screen_w / 2 - BTN_W / 2,
                screen_h / 2 - BTN_H / 2,
                BTN_W,
                BTN_H,
                Color::BLUE,
                "Play".to_string(),
            ),
            quit_button: Button::new(
                screen_w / 2 - BTN_W / 2,
                screen_h / 2 + BTN_H,
                BTN_W,
                BTN_H,
                Color::BLUE,
                "Quit".to_string(),
            ),
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, rl: &RaylibHandle) -> Transition {
        if self.play_button.is_clicked(rl) {
            info!("Play button clicked");
            return Transition::None; // replace with Push(GameScene) later
        }
        if self.quit_button.is_clicked(rl) {
            info!("Quit button clicked");
            return Transition::Quit;
        }
        Transition::None
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread) {
        let bg = &self.background;
        let src = Rectangle::new(0.0, 0.0, bg.width() as f32, bg.height() as f32);
        let dst = Rectangle::new(
            0.0,
            0.0,
            d.get_screen_width() as f32,
            d.get_screen_height() as f32,
        );
        d.draw_texture_pro(bg, src, dst, Vector2::zero(), 0.0, Color::WHITE);

        d.draw_text(
            "Aetherforge",
            d.get_screen_width() / 2 - 100,
            50,
            40,
            Color::WHITE,
        );
        self.play_button.draw(d);
        self.quit_button.draw(d);
        d.draw_fps(10, 10);
    }
}
