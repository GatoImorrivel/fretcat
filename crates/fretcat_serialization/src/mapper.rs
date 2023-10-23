use fretcat_effects::effects::{Fuzz, Overdrive, AudioEffect};
use fretcat_macros::EffectMapper;

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, EffectMapper, Clone)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
}

#[derive(Debug, Clone, Copy)]
pub enum MapperError {
    NotFound
}