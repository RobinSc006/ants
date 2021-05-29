use glam::DVec2;
use rand::{distributions::Uniform, prelude::Distribution};
use sdl2::rect::Rect;

#[derive(Default)]
pub struct Ant {
    pos: DVec2,

    size: f64,
    speed: f64,
    wander_direction_sway: f64,

    perception_radius: u16,
    pheromone_radius: u16,

    wander_target_dir: DVec2,
}

impl Ant {
    pub fn new(pos: DVec2) -> Self {
        Self {
            pos: pos,

            perception_radius: 10,
            pheromone_radius: 15,

            size: 5.5,
            speed: 3.5,
            wander_direction_sway: 0.2,

            wander_target_dir: DVec2::default(),
        }
    }

    pub fn update(&mut self) {
        // TODO implement logic
        self.explore();
        self.wrap_screen((1000, 1000));
    }

    pub fn move_to(&mut self, target: DVec2) {
        let delta_x = self.pos.x - target.x;
        let delta_y = self.pos.y - target.y;

        let theta_radians = f64::atan2(delta_y, delta_x);

        self.pos += self.angle_to_vec(theta_radians) * self.speed;
    }

    pub fn explore(&mut self) {
        let mut random_gen = rand::thread_rng();
        let random_angle = Uniform::from(0..360);

        let angle = random_angle.sample(&mut random_gen) as f64;

        self.wander_target_dir = (self.wander_target_dir
            + self.angle_to_vec(angle) * self.wander_direction_sway)
            .normalize();

        self.pos += self.wander_target_dir * self.speed;
    }

    pub fn get_render_target(&self) -> Rect {
        return Rect::new(
            self.pos.x as i32 - self.size as i32 / 2,
            self.pos.y as i32 - self.size as i32 / 2,
            self.size as u32,
            self.size as u32,
        );
    }

    fn angle_to_vec(&self, radians: f64) -> DVec2 {
        return -DVec2::new(radians.cos(), radians.sin()).normalize_or_zero();
    }

    fn wrap_screen(&mut self, win_dim: (u32, u32)) {
        if self.pos.x < 0.0 {
            self.pos.x = win_dim.0 as f64;
        } else if self.pos.x > win_dim.0 as f64 {
            self.pos.x = 0.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = win_dim.1 as f64;
        } else if self.pos.y > win_dim.1 as f64 {
            self.pos.y = 0.0;
        }
    }
}
