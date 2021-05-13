use image::{Rgba, RgbaImage};

use crate::{
    marker::{Marker, MarkerType},
    vector::Vector,
};

pub struct MarkerMap {
    markers: Vec<Marker>,
    marker_degradation_rate: f64,
}

impl MarkerMap {
    pub fn new() -> Self {
        return Self {
            markers: Vec::new(),
            marker_degradation_rate: 0.1,
        };
    }
    pub fn update(&mut self) {
        let mut remove_indecies: Vec<usize> = Vec::new();

        for (index, marker) in self.markers.iter_mut().enumerate() {
            marker.intensity -= self.marker_degradation_rate;
            if marker.intensity <= 0.0 {
                remove_indecies.push(index);
            }
        }

        for (shift, index) in remove_indecies.iter().enumerate() {
            self.markers.remove(*index - shift);
        }
    }

    pub fn add_marker(&mut self, marker: Marker) {
        self.markers.push(marker);
    }
    pub fn _get_markers(&self) -> &Vec<Marker> {
        return &self.markers;
    }

    pub fn get_markers_in_zone(&self, pos: Vector, area: f64) -> Vec<Marker> {
        let mut markers: Vec<Marker> = Vec::new();

        for marker in self.markers.iter() {
            if marker.pos.x >= pos.x &&         // right of the left edge AND
                marker.pos.x <= pos.x + area &&    // left of the right edge AND
                marker.pos.y >= pos.y &&         // below the top AND
                marker.pos.y <= pos.y + area
            {
                markers.push(marker.clone()); // above the bottom
            }
        }

        return markers;
    }

    pub fn generate_image(&mut self, image: &mut RgbaImage) {
        image.fill(1);

        for marker in self.markers.iter() {
            if marker.pos.x <= image.width().into()
                && marker.pos.x >= 0.0
                && marker.pos.y <= image.height().into()
                && marker.pos.y >= 0.0
            {
                match marker.marker_type {
                    MarkerType::Explore => {
                        image.put_pixel(
                            marker.pos.x as u32,
                            marker.pos.y as u32,
                            Rgba([255, 0, 0, 255]),
                        );
                    }
                    MarkerType::Return => {
                        image.put_pixel(
                            marker.pos.x as u32,
                            marker.pos.y as u32,
                            Rgba([0, 0, 255, 255]),
                        );
                    }
                }
            }
        }
    }
}
