use std::collections::HashMap;

use nih_plug::vizia::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::systems::*;

pub const EDITOR_WIDTH: u32 = 1260;
pub const EDITOR_HEIGHT: u32 = 848;

lazy_static::lazy_static! {
    pub static ref EFFECT_CARDS: HashMap<EffectKind, Vec<Card>> = {
        let mut hashmap: HashMap<EffectKind, Vec<Card>> = HashMap::new();
        hashmap.insert(EffectKind::Distortion, vec![
            GAIN_BOOSTER_CARD,
            OVERDRIVE_CARD,
            FUZZ_CARD,
            DISTORTION_CARD,
            BIT_CRUSHER_CARD
        ]);

        hashmap.insert(EffectKind::Delay, vec![
            DELAY_CARD,
            TWIN_DELAY_CARD
        ]);
        hashmap.insert(EffectKind::Dynamics, vec![
            AUTO_WAH_CARD,
            LOW_PASS_CARD,
            HIGH_PASS_CARD,
            BAND_PASS_CARD
        ]);
        hashmap.insert(EffectKind::Reverb, vec![
            REVERB_CARD
        ]);

        hashmap
    };

}
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq, Data)]
pub enum EffectKind {
    Distortion,
    Delay,
    Dynamics,
    Reverb,
}

impl EffectKind {
    pub fn variants() -> Vec<Self> {
        EffectKind::iter().fold(vec![], |mut acc, kind| {
            acc.push(kind);
            acc
        })
    }
}

impl ToString for EffectKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}


#[inline]
pub fn normalize(
    value: f32,
    min_input: f32,
    max_input: f32,
    min_output: f32,
    max_output: f32,
) -> f32 {
    let clamped_value = value.max(min_input).min(max_input);

    let input_range = max_input - min_input;
    let output_range = max_output - min_output;
    let normalized = (clamped_value - min_input) * output_range / input_range + min_output;

    normalized
}
