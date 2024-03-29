#[allow(unused)]
mod allpass;
#[allow(unused)]
mod coeffs;
#[allow(unused)]
mod comb;
#[allow(unused)]
mod delayline;
#[allow(unused)]
mod svf_filter;
#[allow(unused)]
mod iir_filter;
#[allow(unused)]
mod reverb;
#[allow(unused)]
mod adsr;
#[allow(unused)]
mod delay;

use std::ops::Range;

pub use self::{
    allpass::AllPass,
    svf_filter::{SvfFilter, FilterMode},
    reverb::Freeverb,
    adsr::Envelope,
    delay::Delay,
    iir_filter::*,
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

#[inline]
fn interpolate(s1: f32, s2: f32, offset: f32) -> f32 {
    let one = 1.0_f32;
    let offset = offset;
    let rhs = offset * s2;
    let lhs = (one - offset) * s1;
    lhs + rhs
}