use crate::app::*;

mod ant;
mod app;
mod colony;
mod config;

fn main() {
    let app = App::init();
    app.run();
}
