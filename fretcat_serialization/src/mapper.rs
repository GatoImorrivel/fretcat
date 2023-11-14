use fretcat_effects::effects::{Fuzz, Overdrive, AudioEffect, StudioReverb, Gain, LowPass, HighPass, BandPass, MonoDelay, TwinDelay, BitCrusher};
use fretcat_macros::EffectMapper;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EffectMapper, Clone, PartialEq)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
    Gain(Gain),
    BitCrusher(BitCrusher),

    LowPass(LowPass),
    HighPass(HighPass),
    BandPass(BandPass),

    MonoDelay(MonoDelay),
    TwinDelay(TwinDelay),

    StudioReverb(StudioReverb),
}

#[derive(Debug, Clone, Copy)]
pub enum MapperError {
    NotFound
}