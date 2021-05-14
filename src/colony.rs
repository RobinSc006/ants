use piston_window::{Event, PistonWindow};

use crate::{
    ant::{Ant, State},
    ant_hill::AntHill,
    color::Theme,
    food::Food,
    marker_map::MarkerMap,
    vector::Vector,
};

pub struct Colony {
    ants: Vec<Ant>,
    ant_hill: AntHill,

    num_ants: u16,

    delta_time: f64,
}

impl Colony {
    pub fn new(
        num_ants: u16,
        ant_pos: (f64, f64),
        speed: f64,
        wander_sway: f64,
        sense_radius: f64,
        pickup_radius: f64,
        marker_radius: f64,
        delta: f64,
        marker_drop_rate: u8,
    ) -> Self {
        let mut colony = Self {
            ants: Vec::new(),
            ant_hill: AntHill::new(Vector::new(ant_pos.0, ant_pos.1), 15.0),
            num_ants: num_ants,

            delta_time: delta,
        };

        colony.populate(
            speed,
            wander_sway,
            sense_radius,
            pickup_radius,
            marker_radius,
            marker_drop_rate,
        );

        return colony;
    }

    pub fn render(&self, window: &mut PistonWindow, event: &Event, theme: &Theme) {
        for ant in self.ants.iter() {
            ant.render(window, event, theme);
        }

        self.ant_hill.render(window, event, theme);
    }

    pub fn update(&mut self, food_on_map: &Vec<Food>, markers_on_map: &mut MarkerMap) {
        for ant in self.ants.iter_mut() {
            // Markers
            if ant.is_wandering() || ant.is_targeting() {
                ant.drop_marker(crate::marker::MarkerType::Explore, markers_on_map);
            } else if ant.is_carrying() {
                ant.drop_marker(crate::marker::MarkerType::Return, markers_on_map);
            }

            // Collision
            if !ant.is_carrying() {
                for (_index, food) in food_on_map.clone().iter().enumerate() {
                    let dist_x = ant.pos.x - food.pos.x;
                    let dist_y = ant.pos.y - food.pos.y;

                    let sum_xy = dist_x * dist_x + dist_y * dist_y;

                    if !ant.is_targeting() {
                        // Check if food is visible
                        if f64::sqrt(sum_xy) <= ant.get_sense_radius() {
                            ant.set_target(food.pos);
                            ant.state = State::Target;
                        }
                    } else {
                        // Check if food is colliding
                        if f64::sqrt(sum_xy) <= ant.get_pickup_radius() {
                            ant.state = State::FollowExplore;
                        }
                    }
                }
            } else {
            }

            ant.update(&markers_on_map);
        }
    }

    fn populate(
        &mut self,
        speed: f64,
        wander_sway: f64,
        sense_radius: f64,
        pickup_radius: f64,
        marker_radius: f64,
        marker_drop_rate: u8,
    ) {
        let spawn_area = (
            self.ant_hill.get_pos(),
            self.ant_hill.get_pos()
                + Vector::new(
                    self.ant_hill.get_radius() / 2.0,
                    self.ant_hill.get_radius() / 2.0,
                ),
        );

        for _ in 0..self.num_ants {
            self.ants.push(Ant::new(
                spawn_area,
                self.delta_time,
                speed,
                wander_sway,
                sense_radius,
                pickup_radius,
                marker_radius,
                marker_drop_rate,
            ));
        }
    }
}
