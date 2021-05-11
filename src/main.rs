extern crate piston_window;

use std::time::Instant;

use app::App;
use piston_window::*;
use world::World;

use crate::config::Config;

mod ant;
mod color;
mod random;
mod vector;
mod world;
mod food;
mod config;
mod app;

fn main() {
    let mut app = App::init();

    app.run();
}
