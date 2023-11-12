use fretcat_effects::effects::{Fuzz, Overdrive, AudioEffect, StudioReverb, Gain, LowPass, HighPass, BandPass};
use fretcat_macros::EffectMapper;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EffectMapper, Clone, PartialEq)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
    StudioReverb(StudioReverb),
    Gain(Gain),

    LowPass(LowPass),
    HighPass(HighPass),
    BandPass(BandPass),
}

#[derive(Debug, Clone, Copy)]
pub enum MapperError {
    NotFound
}