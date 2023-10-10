mod allpass;
mod coeffs;
mod comb;
mod delayline;
mod filter;
mod reverb;

pub use self::{
    allpass::AllPass,
    filter::{Filter, FilterMode},
    reverb::Freeverb
};


 pub fn map_normalized_value(value: f32, min_output: f32, max_output: f32) -> f32 {
    min_output + value * (max_output - min_output)
}