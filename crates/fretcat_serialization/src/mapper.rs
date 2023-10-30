use fretcat_effects::effects::{Fuzz, Overdrive, AudioEffect, StudioReverb};
use fretcat_macros::EffectMapper;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EffectMapper, Clone, PartialEq)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
    StudioReverb(StudioReverb),
}

#[derive(Debug, Clone, Copy)]
pub enum MapperError {
    NotFound
}