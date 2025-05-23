use rand::Rng;

pub fn random_f32_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
