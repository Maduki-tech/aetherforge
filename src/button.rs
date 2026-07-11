use raylib::prelude::*;

#[derive(Debug)]
pub struct Button {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    label: String,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color, label: String) -> Self {
        Button { x, y, width, height, color, label }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        let text_size = d.measure_text(&self.label, 20);
        let text_x = self.x + (self.width - text_size) / 2;
        let text_y = self.y + (self.height - 20) / 2;
        d.draw_text(&self.label, text_x, text_y, 20, Color::WHITE);
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();

        rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }
}
