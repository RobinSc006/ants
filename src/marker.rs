use std::cmp::Ordering;

use image::{Rgba, RgbaImage};

use crate::vector::Vector;

pub const DEFAULT_MARKER_INTENTSITY: f64 = 100.0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MarkerType {
    Explore,
    Return,
}
#[derive(Debug, Copy, Clone)]
pub struct Marker {
    pub pos: Vector,
    /// Not named 'type', because that's build in functionality
    pub marker_type: MarkerType,
    pub intensity: f64,
}
