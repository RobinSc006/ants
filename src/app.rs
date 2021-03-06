use crate::config::Config;
use crate::util::map_pos_to_grid;
use crate::world::World;
use glam::DVec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;
use sdl2::render::BlendMode;

use std::time::Instant;

pub struct App {
    config: Config,
}

impl App {
    pub fn init() -> Self {
        match pretty_env_logger::try_init() {
            Ok(_) => {
                println!("logger initialized");
            }
            Err(e) => {
                println!("logger init failed: {}", &e);
            }
        }
        let conf = Config::load("config.ini");
        return Self { config: conf };
    }

    pub fn run(&self) {
        let mut window_dimensions = (
            self.config.get_parameter("win_size").vals[0] as u32,
            self.config.get_parameter("win_size").vals[1] as u32,
        );

        // * World setup
        let world_grid_size = (
            self.config.get_parameter("grid_dim").vals[0] as u32,
            self.config.get_parameter("grid_dim").vals[1] as u32,
        );
        let mut world = World::new(
            self.config.get_parameter("num_ants").vals[0] as u32,
            world_grid_size,
            &mut window_dimensions,
            self.config.get_parameter("tile_size").vals[0],
            Color::BLACK,
            (
                self.config.get_parameter("food_amount_range").vals[0] as u32,
                self.config.get_parameter("food_amount_range").vals[1] as u32,
            ),
        );

        // ! Graphics setup --

        // SDL setup
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Window creation

        let window = video_subsystem
            .window("Ants 2.0", window_dimensions.0, window_dimensions.1)
            .position_centered()
            .build()
            .unwrap();
        let mut win_canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        // Set window background
        win_canvas.set_draw_color(Color::RGB(
            self.config.get_parameter("background_color").vals[0] as u8,
            self.config.get_parameter("background_color").vals[1] as u8,
            self.config.get_parameter("background_color").vals[2] as u8,
        ));

        win_canvas.set_blend_mode(BlendMode::Blend);

        // Event pump creation
        let mut event_pump = sdl_context.event_pump().unwrap();

        // ! Graphics setup end --

        // Timing setup
        const TIMING_TICK_TIME: u128 = 25;
        let mut timing_tick_clock = Instant::now();

        // Main loop
        'running: loop {
            // ! Update --

            // Sdl events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseButtonDown { x, y, .. } => {
                        world.insert_food(
                            map_pos_to_grid(
                                DVec2::new(x as f64, y as f64),
                                world_grid_size,
                                window_dimensions,
                            ),
                            2000,
                        );
                    }
                    _ => {}
                }
            }

            // Tick update

            if timing_tick_clock.elapsed().as_millis() >= TIMING_TICK_TIME {
                world.update();

                timing_tick_clock = Instant::now();
            }

            // ! Update end --

            // * Render --
            win_canvas.clear();

            // Render
            world.render(&mut win_canvas);

            win_canvas.present();
            // * Render end --

            // Delay
            std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
