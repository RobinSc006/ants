use crate::config::Config;

#[derive(Debug, Copy, Clone)]
pub struct Theme {
    pub window_clear_color: [f32; 4],
    pub ant_color: [f32; 4],
    pub food_color: [f32; 4],
    pub font_color: [f32; 4],
    pub gismo_color: [f32; 4],
}

impl Theme {
    pub fn load(config: &Config) -> Self {
        return Self {
            window_clear_color: vec_to_color_rgb(&config.get_parameter("background_color").vals),
            ant_color: vec_to_color_rgb(&config.get_parameter("ant_color").vals),
            food_color: vec_to_color_rgb(&config.get_parameter("food_color").vals),
            font_color: vec_to_color_rgb(&config.get_parameter("font_color").vals),
            gismo_color: vec_to_color_rgba(&config.get_parameter("gismo_color").vals),
        };
    }
}

pub fn get_color_rgb(rgb: (u8, u8, u8)) -> [f32; 4] {
    let red = map(rgb.0 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let green = map(rgb.1 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let blue = map(rgb.2 as f64, 0.0, 255.0, 0.0, 1.0) as f32;

    return [red, green, blue, 1.0];
}

pub fn get_color_rgba(rgba: (u8, u8, u8, u8)) -> [f32; 4] {
    let red = map(rgba.0 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let green = map(rgba.1 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let blue = map(rgba.2 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let alpha = map(rgba.3 as f64, 0.0, 255.0, 0.0, 1.0) as f32;

    return [red, green, blue, alpha];
}

pub fn map(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
}

pub fn vec_to_color_rgb(vec: &Vec<f64>) -> [f32; 4] {
    if vec.len() >= 3 {
        return get_color_rgb((
            vec[0].trunc() as u8,
            vec[1].trunc() as u8,
            vec[2].trunc() as u8,
        ));
    }
    return get_color_rgb((255, 30, 30));
}

pub fn vec_to_color_rgba(vec: &Vec<f64>) -> [f32; 4] {
    if vec.len() >= 4 {
        return get_color_rgba((
            vec[0].trunc() as u8,
            vec[1].trunc() as u8,
            vec[2].trunc() as u8,
            vec[3].trunc() as u8,
        ));
    }
    return get_color_rgba((255, 30, 30, 20));
}
