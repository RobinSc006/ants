use glam::DVec2;

pub fn map(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
}

pub fn map_pos_to_grid(pos: DVec2, grid_size: (u32, u32), window_size: (u32, u32)) -> (u32, u32) {
    return (
        (map(pos.x, 0.0, window_size.0 as f64, 0.0, grid_size.0 as f64) as u32)
            .clamp(0, grid_size.0 - 1),
        (map(pos.y, 0.0, window_size.1 as f64, 0.0, grid_size.1 as f64) as u32)
            .clamp(0, grid_size.1 - 1),
    );
}
