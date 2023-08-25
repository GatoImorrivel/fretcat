use std::f32::consts::PI;

use convolutions_rs::convolutions::ConvolutionLayer;
use ndarray::prelude::*;
use rayon::prelude::*;
use fundsp::{prelude::*, hacker32::butterpass_hz};

use lazy_static::*;

#[no_mangle]
pub fn process_sample(buffer: &mut [f32]) {
    let freq = 440.0;
    let resonance = 0.25;

    let input = buffer.iter().map(|sample| sample.clone()).collect::<Vec<_>>();
    let mut filter = highpass_hz::<f32, f32>(freq, resonance);

    for (input, output) in input.chunks(64).zip(buffer.chunks_mut(64)) {
        filter.process(input.len(), &[input] , &mut[output]);
    }
}

fn convolve(signal: &[f32], kernel: &[f32]) -> Vec<f32> {
    signal.windows(kernel.len()).map(|window| {
        window.iter().zip(kernel).map(|(x, y)| x * y).sum()
    }).collect()
}