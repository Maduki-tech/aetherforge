use raylib::prelude::*;

pub fn is_clickable_rect(rl: &RaylibHandle, pos: Vector2, size: Vector2) -> bool {
    let mouse = rl.get_mouse_position();
    rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        && mouse.x >= pos.x
        && mouse.x <= pos.x + size.x
        && mouse.y >= pos.y
        && mouse.y <= pos.y + size.y
}
