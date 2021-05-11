use piston_window::{Event, PistonWindow, ellipse};

use crate::{ant::Ant, color, vector::Vector};

pub struct World {
    ants: Vec<Ant>,
    num_ants: u16,
}

impl World {
    pub fn new(num_ants: u16) -> Self {
        let mut world = World {
            ants: Vec::new(),
            num_ants: num_ants,
        };

        world.populate();

        return world;
    }

    pub fn render(&self, window: &mut PistonWindow, event: Event) {
        window.draw_2d(&event, |context, graphics, _device| {
            for ant in self.ants.iter() {
                ellipse(color::get((0, 0, 0)), [ant.pos.x, ant.pos.y, 5.0, 5.0], context.transform, graphics);
            }
            
        });
    }

    pub fn update(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.update();
        }
    }

    fn populate(&mut self) {
        let spawn_area = (Vector::new(100.0, 100.0), Vector::new(500.0, 500.0));

        for _ in 0..self.num_ants {
            self.ants.push(Ant::new(spawn_area));
        }
    }
}