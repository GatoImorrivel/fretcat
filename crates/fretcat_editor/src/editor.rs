use std::sync::Arc;

use fretcat_effects::chain::ChainHandle;
use nih_plug::prelude::*;
use nih_plug_vizia::{vizia::prelude::*, ViziaState, create_vizia_editor};

use crate::{top_bar::top_bar, sidebar::sidebar, effect_view::effect_view};

#[derive(Lens, Clone, Debug)]
pub struct Data {
    pub noise_gate: Arc<AtomicF32>
}

impl Model for Data {}

pub fn create(
    editor_data: Data,
    chain_handle: ChainHandle,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, nih_plug_vizia::ViziaTheming::Custom, move |cx, _| {
        cx.add_stylesheet(include_str!("../css/editor.css")).unwrap();

        editor_data.clone().build(cx);
        chain_handle.clone().build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                sidebar(cx);
                effect_view(cx);
            })
            .class("bottom-row");
            top_bar(cx);
        }).class("wrapper");
    })
}