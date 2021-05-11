use crate::{color, vector::Vector};
#[derive(Debug, Copy, Clone)]
pub struct Food {
    pub pos: Vector,
    pub color: [f32; 4],
}

impl Food {
    pub fn new(pos: Vector) -> Self {
        return Self {
            pos: pos,
            color: color::get_color_rgb((255, 205, 178)),
        };
    }
}
