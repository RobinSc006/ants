use piston_window::{ellipse, rectangle, Event, PistonWindow, Transformed};

use crate::{
    ant::Ant,
    color::{self},
    food::Food,
    random,
    vector::Vector,
};

pub struct World {
    ants: Vec<Ant>,
    food_on_map: Vec<Food>,

    num_ants: u16,
    delta_time: f64,

    color_theme: color::Theme,
    render_debug_gismo: bool,
}

impl World {
    pub fn new(
        num_ants: u16,
        num_food: u16,
        debug_gismo: bool,
        theme: color::Theme,
        delta: f64,
    ) -> Self {
        let mut world = World {
            ants: Vec::new(),
            food_on_map: Vec::new(),
            num_ants: num_ants,
            delta_time: delta,

            render_debug_gismo: debug_gismo,
            color_theme: theme,
        };

        world.populate();
        world.cluster_food(
            num_food,
            (Vector::new(600.0, 600.0), Vector::new(650.0, 650.0)),
        );

        return world;
    }

    pub fn render(&self, window: &mut PistonWindow, event: Event) {
        // render food
        window.draw_2d(&event, |context, graphics, _device| {
            for f in self.food_on_map.iter() {
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

                if self.render_debug_gismo {
                    // Render ant sight
                    ellipse(
                        self.color_theme.gismo_color,
                        [
                            ant.pos.x,
                            ant.pos.y,
                            ant.get_sense_radius() * 2.0,
                            ant.get_sense_radius() * 2.0,
                        ],
                        context.transform.trans(
                            -(ant.get_sense_radius() * 2.0) / 2.0,
                            -(ant.get_sense_radius() * 2.0) / 2.0,
                        ),
                        graphics,
                    );
                }
            }
        });
    }

    pub fn update(&mut self) {
        let mut removed_food: usize = 0;

        for ant in self.ants.iter_mut() {
            if !ant.is_carrying() {
                for (index, food) in self.food_on_map.clone().iter().enumerate() {
                    let dist_x = ant.pos.x - food.pos.x;
                    let dist_y = ant.pos.y - food.pos.y;

                    let sum_xy = dist_x * dist_x + dist_y * dist_y;

                    if !ant.is_targeting() {
                        // Check if food is visible
                        if f64::sqrt(sum_xy) <= ant.get_sense_radius() {
                            ant.set_target(*food);
                        }
                    }

                    if true {
                        // Check if food is colliding
                        if f64::sqrt(sum_xy) <= ant.get_pickup_radius() {
                            ant.collect_food();
                            self.food_on_map.remove(index - removed_food);
                            removed_food += 1;
                        }
                    }
                }
            }

            ant.update();
        }
    }

    fn populate(&mut self) {
        let spawn_area = (Vector::new(100.0, 100.0), Vector::new(500.0, 500.0));

        for _ in 0..self.num_ants {
            self.ants.push(Ant::new(spawn_area, self.delta_time));
        }
    }

    fn cluster_food(&mut self, amount: u16, constraits: (Vector, Vector)) {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        for _ in 0..amount {
            spawn_pos.x = random::num((constraits.0.x as i64, constraits.1.y as i64));
            spawn_pos.y = random::num((constraits.0.x as i64, constraits.1.y as i64));

            self.food_on_map.push(Food::new(spawn_pos));
        }
    }
}
