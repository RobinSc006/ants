use std::collections::LinkedList;

use image::{Rgba, RgbaImage};

use crate::vector::Vector;

#[derive(PartialEq, Eq)]
pub enum MarkerType {
    Explore,
    Return,
}

pub struct Marker {
    pub pos: Vector,
    /// Not named 'type', because that's build in functionality
    pub marker_type: MarkerType,
}

pub fn generate_marker_map(image: &mut RgbaImage, markers: &LinkedList<Marker>) {
    for marker in markers.iter() {
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
