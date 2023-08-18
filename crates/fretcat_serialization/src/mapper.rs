use fretcat_effects::{Fuzz, Overdrive, AudioEffect};
use fretcat_macros::EffectMapper;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EffectMapper)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
}

#[derive(Debug, Clone, Copy)]
pub enum MapperError {
    NotFound
}