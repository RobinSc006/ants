use crate::{color::Theme, marker, marker_map::MarkerMap, vector::Vector};
use crate::{
    marker::{Marker, MarkerType},
    random,
};

#[derive(PartialEq, Eq)]
pub enum State {
    Wander,
    Target,
    FollowReturn,
    FollowExplore,
}

pub struct Ant {
    pub pos: Vector,
    pub vel: Vector,
    pub state: State,

    move_speed: f64,
    wander_direction_sway: f64,

    sense_radius: f64,
    pickup_radius: f64,
    marker_radius: f64,

    /// Not as in traditional delta. I just borrow the term for 'time step'
    delta_time: f64,
    ticks_since_marker: u32,

    desired_wander_dir: Vector,
    targeted_pos: Vector,
}

impl Ant {
    pub fn new(
        spawn_constaints: (Vector, Vector),
        delta: f64,
        speed: f64,
        wander_sway: f64,
        sense_radius: f64,
        pickup_radius: f64,
        marker_radius: f64,
    ) -> Self {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        spawn_pos.x = random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));
        spawn_pos.y = random::num((spawn_constaints.0.x as i64, spawn_constaints.1.y as i64));

        return Self {
            pos: spawn_pos,
            vel: Vector::new(0.0, 0.0),
            state: State::Wander,

            move_speed: speed,
            wander_direction_sway: wander_sway,
            marker_radius: marker_radius,
            sense_radius: sense_radius,
            pickup_radius: pickup_radius,
            delta_time: delta,

            ticks_since_marker: 0,

            desired_wander_dir: Vector::from_angle(random::num((0, 360))).normalize(),
            targeted_pos: Vector::new(0.0, 0.0),
        };
    }

    pub fn render(&self, window: &mut piston_window::PistonWindow, event: &piston_window::Event, color_theme: &Theme) {
        window.draw_2d(event, |context, graphics, _device| {
            let ant_size = (5.0, 3.0);

            let transform = piston_window::Transformed::trans(
                piston_window::Transformed::rot_deg(
                    piston_window::Transformed::trans(context.transform, self.pos.x, self.pos.y),
                    self.get_target_dir(),
                ),
                -ant_size.0 / 2.0,
                -ant_size.1 / 2.0,
            );

            piston_window::rectangle(
                color_theme.ant_color,
                [0.0, 0.0, ant_size.0, ant_size.1],
                transform,
                graphics,
            );
        });
    }

    pub fn update(&mut self) {
        match self.state {
            State::Wander => {
                self.wander();
            }
            State::Target => {
                self.target();
            }
            State::FollowReturn => {}
            State::FollowExplore => {}
        }

        self.ticks_since_marker += 1;
        self.update_pos();
    }

    fn wander(&mut self) {
        let wander_target_dir = Vector::from_angle(random::num((0, 360))).normalize();
        self.desired_wander_dir = (self.desired_wander_dir
            + wander_target_dir.multiply_float(self.wander_direction_sway))
        .normalize();

        self.vel = self.desired_wander_dir.multiply_float(self.move_speed);
    }

    fn target(&mut self) {
        self.vel = Vector::from_angle(self.pos.angle_to(self.targeted_pos));
    }

    pub fn drop_marker(&self, m_type: MarkerType, markers: &mut MarkerMap) {
        markers.add_marker(Marker {
            pos: self.pos,
            marker_type: m_type,
            intensity: marker::DEFAULT_MARKER_INTENTSITY,
        });
    }

    fn update_pos(&mut self) {
        self.pos = self.pos + self.vel.multiply_float(self.delta_time);
    }

    pub fn collect_food(&mut self) {
        self.targeted_pos = Vector::new(0.0, 0.0);
        self.state = State::FollowReturn;
    }

    pub fn set_target(&mut self, target: Vector) {
        self.targeted_pos = target;
        self.state = State::Target;
    }

    pub fn get_target_dir(&self) -> f64 {
        return self.vel.degrees();
    }

    pub fn get_sense_radius(&self) -> f64 {
        return self.sense_radius;
    }

    pub fn get_pickup_radius(&self) -> f64 {
        return self.pickup_radius;
    }

    pub fn get_marker_radius(&self) -> f64 {
        return self.marker_radius;
    }

    pub fn is_targeting(&self) -> bool {
        return self.state == State::Target;
    }

    pub fn is_carrying(&self) -> bool {
        return self.state == State::FollowReturn;
    }

    pub fn is_wandering(&self) -> bool {
        return self.state == State::Wander;
    }

    pub fn should_drop_marker(&mut self, marker_ticks: u32) -> bool {
        if self.ticks_since_marker >= marker_ticks {
            self.ticks_since_marker = 0;
            return true;
        }
        return false;
    }
}
