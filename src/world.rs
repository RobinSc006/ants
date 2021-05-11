use piston_window::{rectangle, Event, PistonWindow, Transformed};

use crate::{ant::Ant, color::{self}, config::{Config, ConfigParameter}, food::Food, random, vector::Vector};

pub struct World {
    ants: Vec<Ant>,
    food: Vec<Food>,

    num_ants: u16,

    color_theme: color::Theme,
}

impl World {
    pub fn new(num_ants: u16, theme: color::Theme) -> Self {
        let mut world = World {
            ants: Vec::new(),
            food: Vec::new(),
            num_ants: num_ants,

            color_theme: theme,
        };

        world.populate();
        world.cluster_food(200, (Vector::new(600.0, 600.0), Vector::new(650.0, 650.0)));

        return world;
    }

    pub fn render(&self, window: &mut PistonWindow, event: Event) {
        // render food
        window.draw_2d(&event, |context, graphics, _device| {
            for f in self.food.iter() {
                rectangle(
                    self.color_theme.food_color,
                    [f.pos.x, f.pos.y, 3.0, 3.0],
                    context.transform,
                    graphics,
                );
            }
        });

        // render ants
        window.draw_2d(&event, |context, graphics, _device| {
            let ant_size = (5.0, 3.0);

            for ant in self.ants.iter() {
                let transform = context
                    .transform
                    .trans(ant.pos.x, ant.pos.y)
                    .rot_deg(ant.get_target_dir())
                    .trans(-ant_size.0 / 2.0, -ant_size.1 / 2.0);

                rectangle(
                    self.color_theme.ant_color,
                    [0.0, 0.0, ant_size.0, ant_size.1],
                    transform,
                    graphics,
                );
            }
        });
    }

    pub fn update(&mut self) {
        for ant in self.ants.iter_mut() {
            for f in self.food.iter_mut() {
                if !ant.is_targeting() {
                    let dist_x = ant.pos.x - f.pos.x;
                    let dist_y = ant.pos.y - f.pos.y;

                    let sum_xy = dist_x * dist_x + dist_y * dist_y;

                    // Check if food visible
                    if f64::sqrt(sum_xy) <= 50.0 {
                        f.color = color::get_color((255, 0, 0));
                        ant.set_target(*f);
                    }
                }
            }

            ant.update();
        }
    }

    fn populate(&mut self) {
        let spawn_area = (Vector::new(100.0, 100.0), Vector::new(500.0, 500.0));

        for _ in 0..self.num_ants {
            self.ants.push(Ant::new(spawn_area));
        }
    }

    fn cluster_food(&mut self, amount: u16, constraits: (Vector, Vector)) {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        for _ in 0..amount {
            spawn_pos.x = random::num((constraits.0.x as i64, constraits.1.y as i64));
            spawn_pos.y = random::num((constraits.0.x as i64, constraits.1.y as i64));

            self.food.push(Food::new(spawn_pos));
        }
    }
}
