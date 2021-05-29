const DEGRADATION_RATE: f64 = 0.01;
const DEFAULT_STRENGTH: f64 = 0.01;

#[derive(Debug, Clone, Copy)]
pub struct Marker {
    pub m_type: u8,
    pub strength: f64,
}

impl Marker {
    pub fn new(m_type: u8) -> Self {
        return Self {
            m_type: m_type,
            strength: DEFAULT_STRENGTH,
        };
    }
    pub fn update(&mut self) {
        self.strength -= DEGRADATION_RATE;
    }
}
