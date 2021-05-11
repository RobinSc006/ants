use app::App;

mod ant;
mod app;
mod color;
mod config;
mod food;
mod random;
mod vector;
mod world;

fn main() {
    let mut app = App::init();
    app.run();
}
