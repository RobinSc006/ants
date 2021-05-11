pub fn get(rgb: (u8, u8, u8)) -> [f32; 4]{
    let red = map(rgb.0 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let green = map(rgb.1 as f64, 0.0, 255.0, 0.0, 1.0) as f32;
    let blue = map(rgb.2 as f64, 0.0, 255.0, 0.0, 1.0) as f32;

    return [red, green, blue, 1.0];
}

pub fn map(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
}