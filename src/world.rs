use image::{ImageBuffer, RgbaImage};
use piston_window::{
    ellipse, rectangle, Event, G2dTexture, PistonWindow, Texture, TextureSettings, Transformed,
};
use std::collections::LinkedList;

use crate::{
    ant::Ant,
    ant_hill::AntHill,
    color::{self},
    food::Food,
    marker::{generate_marker_map, Marker},
    random,
    vector::Vector,
};

pub struct World {
    ants: Vec<Ant>,
    ant_hill: AntHill,

    food_on_map: Vec<Food>,
    markers_on_map: LinkedList<Marker>,
    marker_map_image: RgbaImage,

    num_ants: u16,
    delta_time: f64,
    max_markers: u16,

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

        debug_gismo: bool,
        max_markers: u16,
        theme: color::Theme,
        delta: f64,

        window_dimensions: (u32, u32),
    ) -> Self {
        let mut world = World {
            ants: Vec::new(),
            food_on_map: Vec::new(),
            markers_on_map: LinkedList::new(),
            ant_hill: AntHill::new(Vector::new(ant_pos.0, ant_pos.1), 30.0),

            num_ants: num_ants,
            delta_time: delta,
            max_markers: max_markers,
            marker_map_image: ImageBuffer::new(window_dimensions.0, window_dimensions.1),

            render_debug_gismo: debug_gismo,
            color_theme: theme,
            _window_dimensions: window_dimensions,
        };

        world.populate(speed, wander_sway, sense_radius, pickup_radius);
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
        window.draw_2d(event, |context, graphics, _device| {
            let ant_size = (5.0, 3.0);

            for ant in self.ants.iter() {
                let transform = context
                    .transform
                    .trans(ant.pos.x, ant.pos.y)
                    .rot_deg(ant.get_target_dir())
                    .trans(-ant_size.0 / 2.0, -ant_size.1 / 2.0);

                rectangle(
                    self.color_theme.ant_color,
                    [0.0, 0.0, ant_size.0, ant_size.1],
                    transform,
                    graphics,
                );

                if self.render_debug_gismo {
                    // Render ant sight
                    ellipse(
                        self.color_theme.gismo_color,
                        [
                            ant.pos.x,
                            ant.pos.y,
                            ant.get_sense_radius() * 2.0,
                            ant.get_sense_radius() * 2.0,
                        ],
                        context.transform.trans(
                            -(ant.get_sense_radius() * 2.0) / 2.0,
                            -(ant.get_sense_radius() * 2.0) / 2.0,
                        ),
                        graphics,
                    );
                }
            }
        });

        // render ant hill
        window.draw_2d(event, |context, graphics, _device| {
            ellipse(
                self.color_theme.ant_hill_color,
                [
                    self.ant_hill.get_pos().x,
                    self.ant_hill.get_pos().y,
                    self.ant_hill.get_radius() * 2.0,
                    self.ant_hill.get_radius() * 2.0,
                ],
                context.transform.trans(
                    -(self.ant_hill.get_radius() * 2.0) / 2.0,
                    -(self.ant_hill.get_radius() * 2.0) / 2.0,
                ),
                graphics,
            );
        });
    }

    pub fn update(&mut self) {
        generate_marker_map(&mut self.marker_map_image, &self.markers_on_map);
        self.update_ants();
    }

    fn populate(&mut self, speed: f64, wander_sway: f64, sense_radius: f64, pickup_radius: f64) {
        let spawn_area = (
            self.ant_hill.get_pos(),
            self.ant_hill.get_pos()
                + Vector::new(
                    self.ant_hill.get_radius() / 2.0,
                    self.ant_hill.get_radius() / 2.0,
                ),
        );

        for _ in 0..self.num_ants {
            self.ants.push(Ant::new(
                spawn_area,
                self.delta_time,
                speed,
                wander_sway,
                sense_radius,
                pickup_radius,
            ));
        }
    }

    fn cluster_food(&mut self, amount: u16, constraits: (Vector, Vector)) {
        let mut spawn_pos = Vector::new(0.0, 0.0);

        for _ in 0..amount {
            spawn_pos.x = random::num((constraits.0.x as i64, constraits.1.y as i64));
            spawn_pos.y = random::num((constraits.0.x as i64, constraits.1.y as i64));

            self.food_on_map.push(Food::new(spawn_pos));
        }
    }

    fn update_ants(&mut self) {
        for ant in self.ants.iter_mut() {
            // Markers
            if ant.should_drop_marker(10) {
                if ant.is_wandering() || ant.is_targeting() {
                    ant.drop_marker(crate::marker::MarkerType::Explore, &mut self.markers_on_map);
                } else if ant.is_carrying() {
                    ant.drop_marker(crate::marker::MarkerType::Return, &mut self.markers_on_map);
                }
            }

            // Collision
            if !ant.is_carrying() {
                for (_index, food) in self.food_on_map.clone().iter().enumerate() {
                    let dist_x = ant.pos.x - food.pos.x;
                    let dist_y = ant.pos.y - food.pos.y;

                    let sum_xy = dist_x * dist_x + dist_y * dist_y;

                    if !ant.is_targeting() {
                        // Check if food is visible
                        if f64::sqrt(sum_xy) <= ant.get_sense_radius() {
                            ant.set_target(food.pos);
                        }
                    } else {
                        // Check if food is colliding
                        if f64::sqrt(sum_xy) <= ant.get_pickup_radius() {
                            ant.collect_food();
                            ant.set_target(self.ant_hill.get_pos());
                        }
                    }
                }
            } else {
                // TODO implement carry
            }

            ant.update();
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
