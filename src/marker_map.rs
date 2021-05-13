use image::{Rgba, RgbaImage};

use crate::marker::{Marker, MarkerType};

pub struct MarkerMap {
    markers: Vec<Marker>,
}

impl MarkerMap {
    pub fn new() -> Self{
        return Self {
            markers: Vec::new(),
        }
    }
    pub fn add_marker(&mut self, marker: Marker) {
        self.markers.push(marker);
    }
    pub fn get_markers (&self) -> &Vec<Marker> {
        return &self.markers;
    }

    pub fn generate_image(&self, image: &mut RgbaImage,) {
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