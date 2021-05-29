use sdl2::pixels::Color;

use crate::{food::Food, marker::Marker};

pub struct Tile {
    pub markers: (Marker, Marker),
    pub food: Food,
}

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
}