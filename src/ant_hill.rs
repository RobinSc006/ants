use glam::DVec2;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct AntHill {
    pos: DVec2,
    size: f64,
    food_collected: u64,
    color: Color,
}

impl AntHill {
    pub fn new(pos: DVec2, size: f64) -> Self {
        return Self {
            pos: pos,
            size: size,
            food_collected: 0,
            color: Color::BLUE,
        };
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let previous_color = canvas.draw_color();
        canvas.set_draw_color(self.color);

        match canvas.fill_rect(Rect::new(
            self.pos.x as i32 - self.size as i32 / 2,
            self.pos.y as i32 - self.size as i32 / 2,
            self.size as u32,
            self.size as u32,
        )) {
            Ok(_) => {}
            Err(e) => {
                log::error!("render error: {}", e)
            }
        }

        canvas.set_draw_color(previous_color);
    }
}
