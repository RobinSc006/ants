
extern crate piston_window;

use std::time::Instant;

use piston_window::*;
use world::World;

mod vector;
mod ant;
mod world;
mod color;
mod random;

fn main() {
    let window_dimensions = (1000, 1000);

    // Window setup
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("Ants", [window_dimensions.0, window_dimensions.1])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // World setup
    let mut world: World = World::new(10);

    // Tick system setup
    const TICK_TIME: u128 = 25;

    let mut tick_clock = Instant::now();

    // Event loop
    while let Some(event) = window.next() {
        // Tick update
        if tick_clock.elapsed().as_millis() >= TICK_TIME {
            world.update();

            tick_clock = Instant::now();
        }

        // Clear window
        window.draw_2d(&event, |_context, graphics, _device| {
            clear(color::get((89, 77, 67)), graphics);
        });

        // Render ants
        world.render(&mut window, event);

    }
}