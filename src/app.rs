use crate::{config::Config, world::WorldStats};

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
        let mut texture_context = window.create_texture_context();
        let texture_settings = TextureSettings::new();

        // Font loading
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = window
            .load_font(assets.join("font/PTSans-Regular.ttf"))
            .unwrap();

        glyphs.preload_printable_ascii(15).unwrap();

        // Tick system setup
        let tick_time: u128 = self.config.get_parameter("tick_time").vals[0] as u128;
        let mut tick_clock = Instant::now();

        let mut simulation_paused = false;

        // World creation
        let mut world: World = World::new(
            self.config.get_parameter("num_ants").vals[0] as u16,
            self.config.get_parameter("num_food").vals[0] as u16,
            (
                self.config.get_parameter("ant_pos").vals[0],
                self.config.get_parameter("ant_pos").vals[1],
            ),
            self.config.get_parameter("ant_speed").vals[0],
            self.config.get_parameter("ant_wander_sway").vals[0],
            self.config.get_parameter("ant_sense_radius").vals[0],
            self.config.get_parameter("ant_pickup_radius").vals[0],
            self.config.get_parameter("ant_marker_radius").vals[0],
            self.config.get_parameter("marker_max_intensity").vals[0],
            self.config.get_parameter("marker_degradation_rate").vals[0],
            self.config.get_parameter("ant_marker_resolution").vals[0] as u8,
            self.config.get_parameter("debug").vals[0] as u8 != 0,
            color_theme,
            self.config.get_parameter("delta_time").vals[0] as f64,
            window_dimensions,
        );

        let mut world_stats: WorldStats = WorldStats::empty();

        // Event loop
        while let Some(event) = window.next() {
            // Input
            if let Some(button) = event.release_args() {
                match button {
                    Button::Keyboard(key) => {
                        if key == Key::Space {
                            simulation_paused = !simulation_paused;
                        }
                    }
                    Button::Mouse(_) => {}
                    Button::Controller(_) => {}
                    Button::Hat(_) => {}
                }
            }

            // Tick update
            if !simulation_paused && tick_clock.elapsed().as_millis() >= tick_time {
                world.update();
                world_stats = world.get_stats();

                tick_clock = Instant::now();
            }

            // Clear window
            window.draw_2d(&event, |_context, graphics, _device| {
                clear(color_theme.window_clear_color, graphics);
            });

            // Render ants
            world.render(&mut window, &event, &mut texture_context, &texture_settings);

            // Render text
            window.draw_2d(&event, |context, graphics, _device| {
                glyphs.factory.encoder.flush(_device);

                // ? Num ants
                text::Text::new_color(color_theme.font_color, 15)
                    .draw(
                        &format!("Ants: {}", world_stats.num_ants),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(3.0, 15.0),
                        graphics,
                    )
                    .unwrap();

                // ? Num markers
                text::Text::new_color(color_theme.font_color, 15)
                    .draw(
                        &format!("Markers: {}", world_stats.num_pheromones),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(3.0, 30.0),
                        graphics,
                    )
                    .unwrap();

                // ? Num food collected
                text::Text::new_color(color_theme.font_color, 15)
                    .draw(
                        &format!("Food collected: {}", world_stats.num_food_collected),
                        &mut glyphs,
                        &context.draw_state,
                        context.transform.trans(3.0, 45.0),
                        graphics,
                    )
                    .unwrap();

                if simulation_paused {
                    // ? Is paused
                    text::Text::new_color(color::get_color_rgba((255, 20, 20, 255)), 16)
                        .draw(
                            "Paused",
                            &mut glyphs,
                            &context.draw_state,
                            context
                                .transform
                                .trans((window_dimensions.0 / 2) as f64 - 30.0, 20.0),
                            graphics,
                        )
                        .unwrap();
                }
            });
        }
    }
}
