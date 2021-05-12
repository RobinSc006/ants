use app::App;

mod ant;
mod ant_hill;
mod app;
mod color;
mod config;
mod food;
mod marker;
mod random;
mod vector;
mod world;

fn main() {
    let mut app = App::init();
    app.run();
}
