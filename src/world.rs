use image::{ImageBuffer, RgbaImage};
use piston_window::{
    rectangle, Event, G2dTexture, PistonWindow, Texture, TextureSettings,
};

use crate::{colony::{Colony}, color::{self}, food::Food, marker_map::MarkerMap, random, vector::Vector};

pub struct World {
    colony: Colony,

    food_on_map: Vec<Food>,
    marker_map: MarkerMap,
    marker_map_image: RgbaImage,

    color_theme: color::Theme,
    render_debug_gismo: bool,

    _window_dimensions: (u32, u32),
}

impl World {
    pub fn new(
        num_ants: u16,
        num_food: u16,
        ant_pos: (f64, f64),
        speed: f64,
        wander_sway: f64,
        sense_radius: f64,
        pickup_radius: f64,
        marker_radius: f64,

        debug_gismo: bool,
        max_markers: u16,
        theme: color::Theme,
        delta: f64,

        window_dimensions: (u32, u32),
    ) -> Self {
        let mut world = World {
            colony: Colony::new(num_ants, ant_pos, speed, wander_sway, sense_radius, pickup_radius, marker_radius, delta),
            food_on_map: Vec::new(),
            marker_map: MarkerMap::new(),

            marker_map_image: ImageBuffer::new(window_dimensions.0, window_dimensions.1),

            render_debug_gismo: debug_gismo,
            color_theme: theme,
            _window_dimensions: window_dimensions,
        };

        world.cluster_food(
            num_food,
            (Vector::new(600.0, 600.0), Vector::new(650.0, 650.0)),
        );

        return world;
    }

    pub fn render(
        &self,
        window: &mut PistonWindow,
        event: &Event,
        texture_context: &mut piston_window::G2dTextureContext,
        texture_settings: &TextureSettings,
    ) {
        // render food
        window.draw_2d(event, |context, graphics, _device| {
            for f in self.food_on_map.iter() {
                rectangle(
                    self.color_theme.food_color,
                    [f.pos.x, f.pos.y, 3.0, 3.0],
                    context.transform,
                    graphics,
                );
            }
        });

        // render marker map
        window.draw_2d(event, |context, graphics, _device| {
            piston_window::image(
                &self.get_marker_map_texture(texture_context, texture_settings),
                context.transform,
                graphics,
            );
        });

        // render ants
        self.colony.render(window, event, &self.color_theme);
        
    }

    pub fn update(&mut self) {
        self.marker_map.generate_image(&mut self.marker_map_image);
        self.colony.update(&self.food_on_map, &mut self.marker_map);
    }

    fn cluster_food(&mut self, amount: u16, constraits: (Vector, Vector)) {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        for _ in 0..amount {
            spawn_pos.x = random::num((constraits.0.x as i64, constraits.1.y as i64));
            spawn_pos.y = random::num((constraits.0.x as i64, constraits.1.y as i64));

            self.food_on_map.push(Food::new(spawn_pos));
        }
    }

    fn get_marker_map_texture(
        &self,
        texture_context: &mut piston_window::G2dTextureContext,
        texture_settings: &TextureSettings,
    ) -> G2dTexture {
        return Texture::from_image(texture_context, &self.marker_map_image, &texture_settings)
            .unwrap();
    }
}
