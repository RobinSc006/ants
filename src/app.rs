use crate::colony::Colony;
use crate::config::Config;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::pixels::Color;

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
        // ! Graphics setup --

        // SDL setup
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Window creation
        let window_dimensions = (
            self.config.get_parameter("win_size").vals[0] as u32,
            self.config.get_parameter("win_size").vals[1] as u32,
        );
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

        // Event pump creation
        let mut event_pump = sdl_context.event_pump().unwrap();

        // ! Graphics setup end --

        // Timing setup
        const TIMING_TICK_TIME: u128 = 25;
        let mut timing_tick_clock = Instant::now();

        // World setup
        let colony = Colony::new(100, Color::BLACK, (100, 100), (500, 500));

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
                    _ => {}
                }
            }

            // Tick update

            if timing_tick_clock.elapsed().as_millis() >= TIMING_TICK_TIME {
                // Update

                timing_tick_clock = Instant::now();
            }

            // ! Update end --

            // * Render --
            win_canvas.clear();

            // Render
            colony.render(&mut win_canvas);

            win_canvas.present();
            // * Render end --

            // Delay
            std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));

            //
        }
    }
}
