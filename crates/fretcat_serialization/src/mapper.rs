use fretcat_effects::{Fuzz, Overdrive, AudioEffect};
use fretcat_macros::mapper_match;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mapper {
    Overdrive(Overdrive),
    Fuzz(Fuzz),
    None
}

impl From<Box<dyn AudioEffect>> for Mapper {
    fn from(value: Box<dyn AudioEffect>) -> Self {
        mapper_match!(Overdrive);
        mapper_match!(Fuzz);
        Self::None
    }
}