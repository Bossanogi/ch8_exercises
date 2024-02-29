use super::{LENGTH, TOP};
use rand::prelude::*;


pub fn done(nums: &[i32; LENGTH]) -> (f32, f32, bool) {
    let mut errc = false;
    if nums.len() != 0 {
        errc = true;
    }

    let median = takemedian(nums);
    let mode = takemode(nums);
    
    (median, mode, errc)
}

pub fn generate() -> [i32; LENGTH] {
    let mut rng = thread_rng();
    let mut numbers: [i32; LENGTH] = [0; LENGTH];
    for i in 0..LENGTH {
        numbers[i] = rng.gen_range(0..TOP);
    }
    numbers
}

pub fn generate_vec(length: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..length {
        v.push(rng.gen_range(0..100));
    }
    v
}

fn takemedian(nums: &[i32; LENGTH]) -> f32 {
    nums.len() as f32 * 2.0
}

fn takemode(nums: &[i32; LENGTH]) -> f32 {
    nums.len() as f32  * 3.0
}