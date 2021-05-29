use glam::DVec2;
use rand::{distributions::Uniform, prelude::Distribution};
use sdl2::rect::Rect;

use crate::{
    ant_hill::AntHill,
    tile::Tile,
    util::{map},
};

const STATE_WANDER: u8 = 0;
const STATE_TARGET_FOOD: u8 = 1;
const STATE_SEARCH_BACK: u8 = 2;
const STATE_TARGET_HOME: u8 = 3;

#[derive(Default)]
pub struct Ant {
    pos: DVec2,

    size: f64,
    speed: f64,
    wander_direction_sway: f64,

    state: u8,

    perception_radius: u32,
    pheromone_radius: u32,
    act_perception_radius: f64,

    marker_drop_strength: f64,

    current_target_tile: (u32, u32),
    wander_target_dir: DVec2,
}

impl Ant {
    pub fn new(pos: DVec2) -> Self {
        Self {
            pos: pos,

            state: 0,

            act_perception_radius: 75.0,
            perception_radius: 10,
            pheromone_radius: 15,

            size: 5.5,
            speed: 3.5,
            wander_direction_sway: 0.2,

            marker_drop_strength: 5.0,

            current_target_tile: (0, 0),
            wander_target_dir: DVec2::default(),
        }
    }

    pub fn update(
        &mut self,
        win_dim: (u32, u32),
        grid_size: (u32, u32),
        world_tiles: &mut Vec<Vec<Tile>>,
        ant_hill: &AntHill,
    ) {
        // TODO implement logic
        match self.state {
            STATE_TARGET_FOOD => {
                self.approach_food(grid_size, win_dim);
                //self.drop_explore_marker(world_tiles, grid_size, win_dim);
            }
            STATE_TARGET_HOME => {
                self.approach_home(grid_size, win_dim, ant_hill);
            }
            STATE_SEARCH_BACK => {
                self.follow_marker(0, world_tiles, grid_size, win_dim);
                self.search_for_home(ant_hill.pos);
            }
            _ => {
                self.explore(world_tiles, grid_size, win_dim);
                self.drop_explore_marker(world_tiles, grid_size, win_dim);
            }
        }
        self.wrap_screen(win_dim);
    }

    pub fn move_to(&mut self, target: DVec2) {
        let delta_x = self.pos.x - target.x;
        let delta_y = self.pos.y - target.y;

        let theta_radians = f64::atan2(delta_y, delta_x);

        self.pos += self.angle_to_vec(theta_radians) * self.speed;
    }

    pub fn explore(
        &mut self,
        world_tiles: &mut Vec<Vec<Tile>>,
        grid_size: (u32, u32),
        win_dim: (u32, u32),
    ) {
        self.wander();
        self.search_for_food(world_tiles, grid_size, win_dim);
    }

    fn wander(&mut self) {
        let mut random_gen = rand::thread_rng();
        let random_angle = Uniform::from(0..360);

        let angle = random_angle.sample(&mut random_gen) as f64;

        self.wander_target_dir = (self.wander_target_dir
            + self.angle_to_vec(angle) * self.wander_direction_sway)
            .normalize();

        self.pos += self.wander_target_dir * self.speed;
    }

    fn search_for_food(
        &mut self,
        world_tiles: &Vec<Vec<Tile>>,
        grid_size: (u32, u32),
        win_dim: (u32, u32),
    ) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        let mut max_food_in_sight = 0;
        let mut found_food = false;

        for x in grid_x as i32 - self.perception_radius as i32
            ..grid_x as i32 + self.perception_radius as i32
        {
            for y in grid_y as i32 - self.perception_radius as i32
                ..grid_y as i32 + self.perception_radius as i32
            {
                if y.is_positive()
                    && x.is_positive()
                    && y < grid_size.1 as i32
                    && x < grid_size.0 as i32
                {
                    if world_tiles[x as usize][y as usize].food.concentration > max_food_in_sight {
                        max_food_in_sight = world_tiles[x as usize][y as usize].food.concentration;

                        self.current_target_tile = (x as u32, y as u32);
                        found_food = true;
                    }
                }
            }
        }

        if found_food {
            self.state = STATE_TARGET_FOOD;
        }
    }

    // ? Well, at least it's O(n)
    fn search_for_home(&mut self, ant_hill_pos: DVec2) {
        if self.pos.distance(ant_hill_pos) <= self.act_perception_radius {
            self.state = STATE_TARGET_HOME;
        }
    }

    fn approach_home(&mut self, grid_size: (u32, u32), win_dim: (u32, u32), ant_hill: &AntHill) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        let (home_x, home_y) = ant_hill.map_pos_to_grid(grid_size, win_dim);

        if grid_x != home_x && grid_y != home_y {
            self.move_to(self.map_target_to_pos((home_x, home_y), grid_size, win_dim));
        } else {
            self.state = STATE_WANDER;
        }
    }

    fn drop_explore_marker(
        &self,
        world_tiles: &mut Vec<Vec<Tile>>,
        grid_size: (u32, u32),
        win_dim: (u32, u32),
    ) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        world_tiles[grid_x as usize][grid_y as usize]
            .markers
            .0
            .m_type = 1;
        world_tiles[grid_x as usize][grid_y as usize]
            .markers
            .0
            .strength += self.marker_drop_strength;
    }

    fn approach_food(&mut self, grid_size: (u32, u32), win_dim: (u32, u32)) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        if grid_x != self.current_target_tile.0 && grid_y != self.current_target_tile.1 {
            self.move_to(self.map_target_to_pos(self.current_target_tile, grid_size, win_dim));
        } else {
            self.state = STATE_SEARCH_BACK;
        }
    }

    fn follow_marker(
        &mut self,
        _m_type: u8,
        world_tiles: &Vec<Vec<Tile>>,
        grid_size: (u32, u32),
        win_dim: (u32, u32),
    ) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        let mut strongest_marker_in_sight = 0.0;
        let mut found_marker = false;

        let mut marker_pos: (u32, u32) = (0, 0);

        for x in grid_x as i32 - self.pheromone_radius as i32
            ..grid_x as i32 + self.pheromone_radius as i32
        {
            for y in grid_y as i32 - self.pheromone_radius as i32
                ..grid_y as i32 + self.pheromone_radius as i32
            {
                if y.is_positive()
                    && x.is_positive()
                    && y < grid_size.1 as i32
                    && x < grid_size.0 as i32
                {
                    if world_tiles[x as usize][y as usize].markers.0.strength
                        > strongest_marker_in_sight
                    {
                        strongest_marker_in_sight =
                            world_tiles[x as usize][y as usize].markers.0.strength;

                        marker_pos = (x as u32, y as u32);
                        found_marker = true;
                    }
                }
            }
        }

        if found_marker {
            self.move_to_grid_pos(marker_pos, grid_size, win_dim);
        }
    }

    fn move_to_grid_pos(&mut self, grid: (u32, u32), grid_size: (u32, u32), win_dim: (u32, u32)) {
        let (grid_x, grid_y) = self.map_pos_to_grid(grid_size, win_dim);

        if grid_x != grid.0 && grid_y != grid.1 {
            self.move_to(self.map_target_to_pos(grid, grid_size, win_dim));
        }
    }

    pub fn map_pos_to_grid(&self, grid_size: (u32, u32), window_size: (u32, u32)) -> (u32, u32) {
        return (
            (map(
                self.pos.x,
                0.0,
                window_size.0 as f64,
                0.0,
                grid_size.0 as f64,
            ) as u32)
                .clamp(0, grid_size.0 - 1),
            (map(
                self.pos.y,
                0.0,
                window_size.1 as f64,
                0.0,
                grid_size.1 as f64,
            ) as u32)
                .clamp(0, grid_size.1 - 1),
        );
    }

    pub fn map_target_to_pos(
        &self,
        target: (u32, u32),
        grid_size: (u32, u32),
        window_size: (u32, u32),
    ) -> DVec2 {
        return DVec2::new(
            (map(
                target.0 as f64,
                0.0,
                grid_size.0 as f64,
                0.0,
                window_size.0 as f64,
            ) as f64)
                .clamp(0.0, window_size.0 as f64),
            (map(
                target.1 as f64,
                0.0,
                grid_size.1 as f64,
                0.0,
                window_size.1 as f64,
            ) as f64)
                .clamp(0.0, window_size.1 as f64),
        );
    }

    pub fn get_render_target(&self) -> Rect {
        return Rect::new(
            self.pos.x as i32 - self.size as i32 / 2,
            self.pos.y as i32 - self.size as i32 / 2,
            self.size as u32,
            self.size as u32,
        );
    }

    fn angle_to_vec(&self, radians: f64) -> DVec2 {
        return -DVec2::new(radians.cos(), radians.sin()).normalize_or_zero();
    }

    fn wrap_screen(&mut self, win_dim: (u32, u32)) {
        if self.pos.x < 0.0 {
            self.pos.x = win_dim.0 as f64;
        } else if self.pos.x > win_dim.0 as f64 {
            self.pos.x = 0.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = win_dim.1 as f64;
        } else if self.pos.y > win_dim.1 as f64 {
            self.pos.y = 0.0;
        }
    }

    pub fn set_pos(&mut self, pos: DVec2) {
        self.pos = pos;
    }
}
