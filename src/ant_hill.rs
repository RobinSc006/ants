use crate::vector::Vector;

#[allow(dead_code)]
pub struct AntHill {
    pos: Vector,
    food_amount: u64,
    radius: f64,
}

#[allow(dead_code)]
impl AntHill {
    pub fn new(pos: Vector, size: f64) -> Self {
        return Self {
            pos: pos,
            radius: size,

            food_amount: 0,
        };
    }

    pub fn add_food(&mut self) {
        self.food_amount += 1;
    }

    pub fn get_pos(&self) -> Vector {
        return self.pos;
    }
    pub fn get_food_amount(&self) -> u64 {
        return self.food_amount;
    }
    pub fn get_radius(&self) -> f64 {
        return self.radius;
    }
}
