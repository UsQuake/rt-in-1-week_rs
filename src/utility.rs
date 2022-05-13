use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
pub fn degrees_to_radians(degrees: f64) -> f64
{
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64  
{
    let mut rng = rand::thread_rng();
    let return_double = rng.gen();
    return_double
}

pub fn random_double_from_seed(seed: &mut ChaCha8Rng) -> f64
{
   // let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let k: f64 = seed.gen();
    return k;
}

pub fn random_double_range(min:f64, max: f64) -> f64
{
    min + (max - min) * random_double()
}

pub fn random_double_range_from_seed(min:f64, max: f64, seed : &mut ChaCha8Rng) -> f64
{
    min + (max - min) * random_double_from_seed(seed)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64
{
    if x < min
    {
        return min
    }
    if x > max
    {
        return max
    }
    
    x
}