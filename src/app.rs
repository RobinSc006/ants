use crate::{ config::Config};

extern crate piston_window;

use std::time::Instant;

use piston_window::*;

use crate::color;
use crate::world::World;

pub struct App {
    config: Config,
}

impl App {
    pub fn init() -> Self {
        return App {
            config: Config::load("config.ini"),
        };
    }

    pub fn run(&mut self) {
        // Color theme loading
        let color_theme = color::Theme::load(&self.config);

        // Window setup
        let window_dimensions: (u32, u32) = (
            self.config.get_parameter("window_size").vals[0] as u32,
            self.config.get_parameter("window_size").vals[1] as u32,
        );

        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new("Ants", [window_dimensions.0, window_dimensions.1])
                .exit_on_esc(true)
                .graphics_api(opengl)
                .build()
                .unwrap();

        // Tick system setup
        let tick_time: u128 = self.config.get_parameter("tick_time").vals[0] as u128;
        let mut tick_clock = Instant::now();

        // World creation
        let mut world: World = World::new(self.config.get_parameter("num_ants").vals[0] as u16, color_theme);

        // Event loop
        while let Some(event) = window.next() {
            // Tick update
            if tick_clock.elapsed().as_millis() >= tick_time {
                world.update();

                tick_clock = Instant::now();
            }

            // Clear window
            window.draw_2d(&event, |_context, graphics, _device| {
                clear(color_theme.window_clear_color, graphics);
            });

            // Render ants
            world.render(&mut window, event);
        }
    }
}
