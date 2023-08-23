use std::f32::consts::PI;

use convolutions_rs::convolutions::ConvolutionLayer;
use ndarray::prelude::*;
use rayon::prelude::*;
use fundsp::prelude::*;

#[no_mangle]
pub fn process_sample(buffer: &mut [f32]) {
    let blend = 1.0;
    let gain =  100.0;
    buffer.par_iter_mut().for_each(|sample| {
        let clean = *sample;
        let amplified = *sample * gain;
        let distorted = (2.0 / PI) * f32::atan(amplified) * gain;

        *sample = (distorted * blend) + (clean * (1.0 - blend));
    });
}

fn convolve(signal: &[f32], kernel: &[f32]) -> Vec<f32> {
    signal.windows(kernel.len()).map(|window| {
        window.iter().zip(kernel).map(|(x, y)| x * y).sum()
    }).collect()
}