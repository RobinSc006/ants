#[allow(dead_code)]
const DEGRADATION_RATE: f64 = 0.5;
const DEFAULT_STRENGTH: f64 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub m_type: u8,
    pub strength: f64,
}
#[allow(dead_code)]
impl Marker {
    pub fn new(m_type: u8) -> Self {
        return Self {
            m_type: m_type,
            strength: DEFAULT_STRENGTH,
        };
    }
    pub fn update(&mut self) {
        if self.m_type != 0 {
            self.strength = (self.strength - DEGRADATION_RATE).clamp(0.0, DEFAULT_STRENGTH);
        }
    }
}
