use glam::DVec2;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::util::*;

pub struct AntHill {
    pub pos: DVec2,
    size: f64,
    pub food_collected: u64,
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

    pub fn map_pos_to_grid(&self, grid_size: (u32, u32), window_size: (u32, u32)) -> (u32, u32) {
        return (
            (map(
                self.pos.x,
                0.0,
                window_size.0 as f64,
                0.0,
                grid_size.0 as f64,
            ) as u32)
                .clamp(0, grid_size.0 - 1),
            (map(
                self.pos.y,
                0.0,
                window_size.1 as f64,
                0.0,
                grid_size.1 as f64,
            ) as u32)
                .clamp(0, grid_size.1 - 1),
        );
    }
}
