use std::f32::consts::PI;

use convolutions_rs::convolutions::ConvolutionLayer;
use ndarray::prelude::*;
use rayon::prelude::*;
use fundsp::{prelude::*, hacker32::butterpass_hz};

use lazy_static::*;

#[no_mangle]
pub fn process_sample(buffer: &mut [f32]) {
    let mut butter = ButterLowpass::<f32, f32, U1>::new(100.0);
    butter.set_sample_rate(48000.0);

    let aux = buffer.iter().map(|s| *s).collect::<Vec<_>>();

    buffer.chunks_mut(MAX_BUFFER_SIZE).zip(aux.chunks(MAX_BUFFER_SIZE)).for_each(|(chunk, aux_chunk)| {
        butter.process(MAX_BUFFER_SIZE, &[aux_chunk], &mut [chunk]);
    });
}

fn convolve(signal: &[f32], kernel: &[f32]) -> Vec<f32> {
    signal.windows(kernel.len()).map(|window| {
        window.iter().zip(kernel).map(|(x, y)| x * y).sum()
    }).collect()
}