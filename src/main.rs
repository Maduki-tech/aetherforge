mod button;
mod util;

use raylib::prelude::*;

const BTN_W: i32 = 200;
const BTN_H: i32 = 50;

fn main() {
    let (mut rl, thread) = raylib::init().fullscreen().title("Aetherforge").build();

    let screen_w = rl.get_screen_width();
    let screen_h = rl.get_screen_height();

    let ok_button = button::Button::new(
        screen_w / 2 - BTN_W / 2,
        screen_h / 2 - BTN_H / 2,
        BTN_W,
        BTN_H,
        Color::BLUE,
        "OK".to_string(),
    );

    let quit_button = button::Button::new(
        screen_w / 2 - BTN_W / 2,
        screen_h / 2 + BTN_H,
        BTN_W,
        BTN_H,
        Color::BLUE,
        "Quit".to_string(),
    );

    // gameloop
    while !rl.window_should_close() {
        if ok_button.is_clicked(&rl) {
            println!("OK clicked");
        }

        if quit_button.is_clicked(&rl) {
            println!("Quit clicked");
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        ok_button.draw(&mut d);
        quit_button.draw(&mut d);

        d.draw_fps(10, 10);
    }
}
