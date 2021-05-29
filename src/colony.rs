use glam::DVec2;
use rand::{distributions::Uniform, prelude::Distribution};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use rayon::prelude::*;

use crate::{ant::Ant, ant_hill::AntHill};

pub struct Colony {
    ants: Vec<Ant>,
    ant_hill: AntHill,

    ant_color: Color,
}

impl Colony {
    pub fn new(
        num_ants: u32,
        color: Color,
        spawn_area_a: (u32, u32),
        spawn_area_b: (u32, u32),
    ) -> Self {
        // ? Ant spawning
        let mut temp_ants: Vec<Ant> = Vec::new();
        let mut ant_average_pos = DVec2::default();

        let mut random_gen = rand::thread_rng();

        let random_range_x = Uniform::from(spawn_area_a.0..spawn_area_b.0);
        let random_range_y = Uniform::from(spawn_area_a.1..spawn_area_b.1);

        for _ in 0..num_ants {
            let pos = DVec2::new(
                random_range_x.sample(&mut random_gen) as f64,
                random_range_y.sample(&mut random_gen) as f64,
            );
            temp_ants.push(Ant::new(pos));

            ant_average_pos += pos;
        }

        // ? Ant hill pos

        return Self {
            ants: temp_ants,
            ant_hill: AntHill::new(ant_average_pos / num_ants as f64, 25.0),

            ant_color: color,
        };
    }

    pub fn update(&mut self) {
        self.ants.par_iter_mut().for_each(|ant| {
            ant.update();
        })
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let previous_color = canvas.draw_color();
        canvas.set_draw_color(self.ant_color);

        self.render_ants(canvas);
        self.ant_hill.render(canvas);

        canvas.set_draw_color(previous_color);
    }

    fn render_ants(&self, canvas: &mut Canvas<Window>) {
        let mut ant_rects: Vec<Rect> = Vec::new();

        for ant in self.ants.iter() {
            ant_rects.push(ant.get_render_target());
        }

        match canvas.fill_rects(&ant_rects) {
            Ok(_) => {}
            Err(e) => {
                log::error!("render error: {}", &e);
            }
        }
    }
}
