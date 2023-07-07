pub mod editor;
mod sidebar;
mod top_bar;
mod effect_view;
mod card;

use std::{sync::Arc, collections::HashMap};
use fretcat_effects::EffectKind;
use nih_plug_vizia::{ViziaState, vizia::{view::{View, Handle}, prelude::{TextModifiers, Context}}};

use card::*;

pub const EDITOR_WIDTH: u32 = 1260;
pub const EDITOR_HEIGHT: u32 = 848;

lazy_static::lazy_static! {
    pub static ref EFFECT_CARDS: HashMap<EffectKind, Vec<Card>> = {
        let mut hashmap: HashMap<EffectKind, Vec<Card>> = HashMap::new();
        hashmap.insert(EffectKind::Distortion, vec![
            OVERDRIVE_CARD,
            FUZZ_CARD
        ]);
        hashmap
    };
}

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}