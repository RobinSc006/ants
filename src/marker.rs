use image::{Rgba, RgbaImage};

use crate::vector::Vector;

pub const DEFAULT_MARKER_INTENTSITY: f64 = 10.0;

#[derive(PartialEq, Eq)]
pub enum MarkerType {
    Explore,
    Return,
}

pub struct Marker {
    pub pos: Vector,
    /// Not named 'type', because that's build in functionality
    pub marker_type: MarkerType,
    pub intensity: f64,
}
