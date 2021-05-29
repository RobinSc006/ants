use rand::{distributions::Uniform, prelude::Distribution};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{colony::Colony, food::Food, marker::Marker, tile::Tile};

pub struct World {
    colony: Colony,
    grid: Vec<Vec<Tile>>,
    window_size: (u32, u32),
    grid_size: (u32, u32),
    tile_size: f64,

    inserted_food_coords: Vec<(u32, u32)>,
}

impl World {
    pub fn new(
        colony_size: u32,
        grid_size: (u32, u32),
        window_size: &mut (u32, u32),
        desired_tile_size: f64,
        ant_color: Color,
    ) -> Self {
        let window_x = grid_size.0 as f64 * desired_tile_size;
        let window_y = grid_size.1 as f64 * desired_tile_size;

        *window_size = (window_x.ceil() as u32, window_y.ceil() as u32).clone();

        // Grid init
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let empty_marker = Marker::new(0);
        let empty_food = Food { concentration: 0 };

        // Random food setup
        let mut random_gen = rand::thread_rng();

        let random_food_range_amount = Uniform::from(15..255);
        let random_food_range_num = Uniform::from(2..10);
        let random_food_range_x = Uniform::from(0..grid_size.0);
        let random_food_range_y = Uniform::from(0..grid_size.0);

        // Grid init
        for x in 0..grid_size.0 {
            tiles.push(Vec::new());
            for _ in 0..grid_size.1 {
                tiles[x as usize].push(Tile {
                    markers: (empty_marker, empty_marker),
                    food: empty_food,
                });
            }
        }

        let mut world = Self {
            colony: Colony::new(colony_size, ant_color, (0, 0), *window_size),
            grid: tiles,
            window_size: *window_size,
            tile_size: desired_tile_size,
            grid_size: grid_size,
            inserted_food_coords: Vec::new(),
        };

        // Generate food
        for _ in 0..random_food_range_num.sample(&mut random_gen) {
            world.insert_food(
                (
                    random_food_range_x.sample(&mut random_gen),
                    random_food_range_y.sample(&mut random_gen),
                ),
                random_food_range_amount.sample(&mut random_gen),
            );
        }

        return world;
    }

    pub fn update(&mut self) {
        self.colony.update(
            self.window_size,
            self.grid_size,
            &mut self.grid,
            &mut self.inserted_food_coords,
        );
        //self.update_tiles();
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        self.render_tiles(canvas);
        self.colony.render(canvas);
    }

    fn render_tiles(&self, canvas: &mut Canvas<Window>) {
        let previous_color = canvas.draw_color();

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                canvas.set_draw_color(self.grid[x][y].get_color());

                match canvas.fill_rect(Rect::new(
                    x as i32 * self.tile_size as i32,
                    y as i32 * self.tile_size as i32,
                    self.tile_size as u32,
                    self.tile_size as u32,
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("render error: {}", &e);
                    }
                }
            }
        }

        canvas.set_draw_color(previous_color);
    }

    #[allow(dead_code)]
    fn update_tiles(&mut self) {
        self.grid.par_iter_mut().for_each(|column| {
            column.par_iter_mut().for_each(|tile| {
                tile.update();
            })
        })
    }

    pub fn insert_food(&mut self, grid_pos: (u32, u32), amount: u32) {
        let empty_marker = Marker::new(0);

        self.grid[grid_pos.0 as usize][grid_pos.1 as usize] = Tile {
            markers: (empty_marker, empty_marker),
            food: Food {
                concentration: amount,
            },
        };

        self.inserted_food_coords.push(grid_pos);
    }
}
