use sdl2::pixels::Color;

use crate::{food::Food, marker::Marker};

pub struct Tile {
    pub markers: (Marker, Marker),
    pub food: Food,
}

#[allow(dead_code)]
impl Tile {
    pub fn get_color(&self) -> Color {
        return Color::RGBA(
            self.markers.0.m_type * 100,
            self.food.concentration.clamp(0, 255) as u8,
            self.markers.1.m_type * 100,
            (self.markers.0.strength + self.markers.1.strength + self.food.concentration as f64)
                .clamp(0.0, 255.0) as u8,
        );
    }

    pub fn update(&mut self) {
        self.markers.0.update();
        self.markers.1.update();
    }

    /// Returns false if concentration minus one is negative
    pub fn sub_food(&mut self) -> bool {
        if self.food.concentration as i32 - 1 < 0 {
            return false;
        }
        self.food.concentration -= 1;
        return true;
    }
}
