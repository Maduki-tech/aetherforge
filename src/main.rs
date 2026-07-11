use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Aetherforge").build();

    let rotation_speed: f32 = 180.0;
    let mut ring_angle = 270.0;

    while !rl.window_should_close() {
        ring_angle += rotation_speed * rl.get_frame_time();

        if ring_angle >= 360.0 {
            ring_angle -= 360.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_text("Hello Aetherforge", 100, 100, 40, Color::WHITE);
        d.draw_ring(
            Vector2::new(640.0, 360.0),
            80.0,
            120.0,
            ring_angle - 270.0,
            ring_angle,
            80,
            Color::WHITE,
        );

        d.draw_fps(10, 10);
    }
}
