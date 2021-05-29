use glam::DVec2;
use sdl2::rect::Rect;

#[derive(Default)]
pub struct Ant {
    pos: DVec2,
    perception_radius: u16,
    pheromone_radius: u16,
    size: f64,
}

impl Ant {
    pub fn new(pos: DVec2) -> Self {
        Self {
            pos: pos,
            perception_radius: 10,
            pheromone_radius: 15,
            size: 7.0,
        }
    }

    pub fn get_render_target(&self) -> Rect {
        return Rect::new(
            self.pos.x as i32 - self.size as i32 / 2,
            self.pos.y as i32 - self.size as i32 / 2,
            self.size as u32,
            self.size as u32,
        );
    }
}
