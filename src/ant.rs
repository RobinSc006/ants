use crate::{food::Food, random};
use crate::vector::Vector;
#[derive(PartialEq, Eq)]
pub enum State {
    Wander,
    Target,
}

pub struct Ant {
    pub pos: Vector,
    pub vel: Vector,
    pub state: State,

    move_speed: f64,
    wander_direction_sway: f64,
    sense_radius: f64,

    desired_wander_dir: Vector,
    targeted_food_pos: Vector,
}

impl Ant {
    pub fn new(spawn_constaints: (Vector, Vector)) -> Self {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        spawn_pos.x = random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));
        spawn_pos.y = random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));

        return Self {
            pos: spawn_pos,
            vel: Vector::new(0.0, 0.0),
            state: State::Wander,

            move_speed: 1.0,
            wander_direction_sway: 0.15,
            sense_radius: 50.0,

            desired_wander_dir: Vector::from_angle(random::num((0, 360))).normalize(),
            targeted_food_pos: Vector::new(0.0, 0.0),
        };
    }

    pub fn update(&mut self) {
        match self.state {
            State::Wander => {
                self.wander();
            }
            State::Target => {
                self.target();
            }
        }

        self.update_pos();
    }

    fn wander(&mut self) {
        let wander_target_dir = Vector::from_angle(random::num((0, 360))).normalize();
        self.desired_wander_dir = (self.desired_wander_dir
            + wander_target_dir.multiply_float(self.wander_direction_sway))
        .normalize();

        self.vel = self.desired_wander_dir.multiply_float(self.move_speed);

        // Damn, that actually worked
    }

    fn target(&mut self) {
        let target_angle = f64::atan2(self.targeted_food_pos.y - self.pos.y, self.targeted_food_pos.x - self.pos.x) * 180.0 / std::f64::consts::PI;

        self.vel = Vector::from_angle(-target_angle).normalize().multiply_float(self.move_speed);
    }

    fn update_pos(&mut self) {
        self.pos = self.pos + self.vel;
    }

    pub fn set_target(&mut self, food: Food) {
        self.targeted_food_pos = food.pos;
        self.state = State::Target;
    }

    pub fn get_target_dir(&self) -> f64 {
        return self.vel.degrees();
    }

    pub fn is_targeting(&self) -> bool {
        return self.state == State::Target;
    }
}
