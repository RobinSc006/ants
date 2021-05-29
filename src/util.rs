pub fn map(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2;
}
