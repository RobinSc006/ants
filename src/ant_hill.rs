use piston_window::{ellipse, Event, PistonWindow, Transformed};

use crate::{color::Theme, vector::Vector};

#[allow(dead_code)]
pub struct AntHill {
    pos: Vector,
    food_amount: u64,
    radius: f64,
}

#[allow(dead_code)]
impl AntHill {
    pub fn new(pos: Vector, size: f64) -> Self {
        return Self {
            pos: pos,
            radius: size,

            food_amount: 0,
        };
    }

    pub fn render(&self, window: &mut PistonWindow, event: &Event, color_theme: &Theme) {
        // render ant hill
        window.draw_2d(event, |context, graphics, _device| {
            ellipse(
                color_theme.ant_hill_color,
                [
                    self.get_pos().x,
                    self.get_pos().y,
                    self.get_radius() * 2.0,
                    self.get_radius() * 2.0,
                ],
                context.transform.trans(
                    -(self.get_radius() * 2.0) / 2.0,
                    -(self.get_radius() * 2.0) / 2.0,
                ),
                graphics,
            );
        });
    }

    pub fn add_food(&mut self) {
        self.food_amount += 1;
    }

    pub fn get_pos(&self) -> Vector {
        return self.pos;
    }
    pub fn get_food_amount(&self) -> u64 {
        return self.food_amount;
    }
    pub fn get_radius(&self) -> f64 {
        return self.radius;
    }
}
