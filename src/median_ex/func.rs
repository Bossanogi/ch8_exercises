use core::num;

use super::I;

pub fn done(nums: &[f32; I]) -> (f32, f32, bool) {
    let mut errc = false;
    if nums.len() != 0 {
        errc = true;
    }

    let mut median = takemedian(nums);
    let mut mode = takemode(nums);
    
    (median, mode, errc)
}

pub fn generate() -> [f32; I] {
    let mut numbers: [f32; I] = [0.0; I];
    for i in 0..I {
        numbers[i] = i as f32 + 3.0;
    }
    numbers
}

fn takemedian(nums: &[f32; I]) -> f32 {
    nums.len() as f32 * 2.0
}

fn takemode(nums: &[f32; I]) -> f32 {
    nums.len() as f32  * 3.0
}