mod allpass;
mod coeffs;
mod comb;
mod delayline;
mod filter;
mod reverb;
mod adsr;

use std::ops::Range;

pub use self::{
    allpass::AllPass,
    filter::{Filter, FilterMode},
    reverb::Freeverb,
    adsr::Envelope
};


#[inline]
 pub fn map_normalized_value(value: f32, min_output: f32, max_output: f32) -> f32 {
    min_output + value * (max_output - min_output)
}

#[inline]
pub fn normalize_value(input: f32, range: Range<f32>) -> f32 {
    let clamped_input = input.max(range.start).min(range.end);

    let normalized = (clamped_input - range.start) / (range.end - range.start);

    normalized
}

#[inline]
pub fn rms(buffer: &[f32]) -> f32 {
    (buffer.iter().map(|sample| sample * sample).sum::<f32>() / buffer.len() as f32).sqrt()
}