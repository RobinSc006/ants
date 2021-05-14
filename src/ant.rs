use crate::{color::Theme, marker_map::MarkerMap, vector::Vector};
use crate::{
    marker::{Marker, MarkerType},
    random,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Wander,
    Target,
    Home,
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
    marker_drop_rate: u8,

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
        marker_drop_rate: u8,
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
            marker_drop_rate: marker_drop_rate,

            ticks_since_marker: 0,

            desired_wander_dir: Vector::from_angle(random::num((0, 360))).normalize(),
            targeted_pos: Vector::new(0.0, 0.0),
        };
    }

    pub fn render(
        &self,
        window: &mut piston_window::PistonWindow,
        event: &piston_window::Event,
        color_theme: &Theme,
    ) {
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

    pub fn update(&mut self, markers: &MarkerMap) {
        let nearby_markers = markers.get_markers_in_zone(&self);

        match self.state {
            State::Wander => {
                self.wander();
            }
            State::Target => {
                self.target();
            }
            State::FollowReturn => {
                self.follow_markers(MarkerType::Return, &nearby_markers);
            }
            State::FollowExplore => {
                self.follow_markers(MarkerType::Explore, &nearby_markers);
            }
            State::Home => {
                self.target();
            }
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

    fn follow_markers(&mut self, _marker_type: MarkerType, markers: &Vec<Marker>) {
        let mut fallback_to_wander = true;
        let mut least_intense_marker: Marker = Marker {
            pos: Vector::new(0.0, 0.0),
            marker_type: MarkerType::Return,
            intensity: 100.0,
        };

        for marker in markers.iter() {
            if marker.compare(_marker_type) {
                if marker.intensity < least_intense_marker.intensity {
                    least_intense_marker = *marker;
                    fallback_to_wander = false;
                }
            }
        }

        if fallback_to_wander {
            self.state = State::Wander;
            return;
        }
        self.vel = Vector::from_angle(self.pos.angle_to(least_intense_marker.pos));
    }

    pub fn drop_marker(&mut self, m_type: MarkerType, markers: &mut MarkerMap) {
        if !self.should_drop_marker() {
            return;
        }
        markers.add_marker(m_type, self.pos);
    }

    fn update_pos(&mut self) {
        self.pos = self.pos + self.vel.multiply_float(self.delta_time);
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

    /// The only way to compare Enums apparently
    pub fn state_cmp(&self, other: State) -> bool {
        return &format!("{:?}", other) == &format!("{:?}", self.state);
    }

    pub fn should_drop_marker(&mut self) -> bool {
        if self.ticks_since_marker >= self.marker_drop_rate.into() {
            self.ticks_since_marker = 0;
            return true;
        }
        return false;
    }
}
