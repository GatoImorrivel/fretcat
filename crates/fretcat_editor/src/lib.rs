pub mod editor;
mod sidebar;
mod top_bar;
mod effect_view;
mod card;

use std::{sync::Arc, collections::HashMap};
use fretcat_effects::EffectKind;
use nih_plug_vizia::{ViziaState, vizia::{view::{View, Handle}, prelude::{TextModifiers, Context}}};

pub const EDITOR_WIDTH: u32 = 1260;
pub const EDITOR_HEIGHT: u32 = 848;

lazy_static::lazy_static! {
    pub static ref EFFECT_CARDS: HashMap<EffectKind, Vec<fn(&mut Context)>> = {
        let mut hashmap: HashMap<EffectKind, Vec<fn(&mut Context)>> = HashMap::new();
        hashmap.insert(EffectKind::Distortion, vec![
            card::overdrive_card,
            card::fuzz_card
        ]);
        hashmap
    };
}

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}