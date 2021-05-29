use crate::app::*;

mod ant;
mod ant_hill;
mod app;
mod colony;
mod config;
mod food;
mod marker;
mod tile;
mod world;

fn main() {
    let app = App::init();
    app.run();
}
