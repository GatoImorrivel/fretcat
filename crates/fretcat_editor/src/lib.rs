mod components;
mod systems;
mod common;

use std::sync::Arc;

use common::{EDITOR_WIDTH, EDITOR_HEIGHT, register_styles, EFFECT_CARDS};
use fretcat_effects::{ChainData, Chain};

use nih_plug::prelude::*;
use nih_plug::vizia::prelude::*;
use nih_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::systems::*;
use crate::components::*;

pub type EditorState = ViziaState;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

pub type InitFlags = Arc<Chain>;

pub fn create(
    chain: InitFlags,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        ChainData {
            chain: chain.clone(),
        }
        .build(cx);

        fretcat_effects::register_fonts(cx);
        fretcat_effects::register_images(cx);
        fretcat_effects::register_styles(cx);
        register_styles(cx);

        CardSystem::init(cx);
        MessageSystem::init(cx);

        HStack::new(cx, |cx| {
            Sidebar::new(cx);

            CardList::new(cx);
            PresetList::new(cx);

            EffectList::new(cx, ChainData::chain);
        })
        .class("main");

        CardSystem::view(cx);
    })
}