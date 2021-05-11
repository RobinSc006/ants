pub fn num(range: (i64, i64)) -> f64 {
    use rand::distributions::{Distribution, Uniform};

    let mut random_gen = rand::thread_rng();
    let random_range = Uniform::from(range.0..range.1);

    return random_range.sample(&mut random_gen) as f64;
}