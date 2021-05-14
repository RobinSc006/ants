use crate::vector::Vector;

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

impl Marker {
    // Listen...
    // I know what this looks like, but rust has given me no choice
    // it might also just be my own incompetence, but that's besides
    // the point.. PartialEq, Eq and matches! don't seem to work, so...
    pub fn compare(&self, other: MarkerType) -> bool {
        return &format!("{:?}", other) == &format!("{:?}", self.marker_type);
    }
}
