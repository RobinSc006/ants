use crate::vector::Vector;
use crate::random;

pub struct Ant {
    pub pos: Vector,

    move_speed: f64,
    wander_direction_sway: f64,

    desired_wander_dir: Vector,
}

impl Ant {
    pub fn new(spawn_constaints: (Vector, Vector)) -> Self {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        spawn_pos.x =  random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));
        spawn_pos.y =  random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));

        return Self {
            pos: spawn_pos,
            move_speed: 1.0,
            wander_direction_sway: 0.2,
            desired_wander_dir: Vector::from_angle(random::num((0, 360))).normalize(),
        };
    }

    pub fn update(&mut self) {
        self.wander();
    }

    /// "Complicated" Math
    // Goes here
    fn wander(&mut self) {
        let wander_target_dir = Vector::from_angle(random::num((0, 360))).normalize();
        self.desired_wander_dir = (self.desired_wander_dir + wander_target_dir.multiply_float(self.wander_direction_sway)).normalize();

        self.pos = self.pos + self.desired_wander_dir.multiply_float(self.move_speed);

        // Damn, that actually worked
    }
}
