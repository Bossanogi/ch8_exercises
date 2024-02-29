
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

pub fn done_vec(nums: &Vec<i32>) -> (f32, f32, bool) {
    let median = takemedian_vec(nums);
    let mode = takemode_vec(nums);
    (median, mode, true)
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
    let mut buffer: [i32; LENGTH] = nums.clone();
    buffer.sort();
    println!("{:?}", buffer);
    println!("{}", buffer[LENGTH/2 - 1]);
    buffer[LENGTH/2 - 1] as f32
}

fn takemode(nums: &[i32; LENGTH]) -> f32 {
    nums.len() as f32  * 3.0
}

fn sort_vec(nums: &Vec<i32>) -> Vec<i32> {
    let mut buffer: Vec<i32> = nums.clone();
    buffer.sort();
    buffer
}

fn takemedian_vec(nums: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    for x in nums {
        sum += x;
        count += 1;
    }
    (sum/count) as f32
}

fn takemode_vec(nums: &Vec<i32>) -> f32 {
    10.0
}