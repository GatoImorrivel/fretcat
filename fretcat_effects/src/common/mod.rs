#[allow(unused)]
mod allpass;
#[allow(unused)]
mod coeffs;
#[allow(unused)]
mod comb;
#[allow(unused)]
mod delayline;
#[allow(unused)]
mod filter;
#[allow(unused)]
mod reverb;
#[allow(unused)]
mod adsr;

use std::ops::Range;

pub use self::{
    allpass::AllPass,
    filter::{Filter, FilterMode},
    reverb::Freeverb,
    adsr::Envelope
};


#[inline]
pub fn normalize_value(input: f32, range: &Range<f32>) -> f32 {
    let clamped_input = input.max(range.start).min(range.end);

    let normalized = (clamped_input - range.start) / (range.end - range.start);

    normalized
}

#[inline]
pub fn map_value(value: f32, min_input: f32, max_input: f32, min_output: f32, max_output: f32) -> f32 {
    // Ensure the input value is within the specified range
    let clamped_value = value.clamp(min_input, max_input);

    // Perform linear mapping
    let input_range = max_input - min_input;
    let output_range = max_output - min_output;

    let normalized_value = (clamped_value - min_input) / input_range;
    let mapped_value = min_output + normalized_value * output_range;

    mapped_value
}

#[inline]
pub fn rms(buffer: &[f32]) -> f32 {
    (buffer.iter().map(|sample| sample * sample).sum::<f32>() / buffer.len() as f32).sqrt()
}