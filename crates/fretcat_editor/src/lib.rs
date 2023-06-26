mod views;

use std::sync::Arc;

use fretcat_audio::chain::ChainHandle;
use nih_plug::nih_log;
use nih_plug::prelude::{Editor, AtomicF32};
use nih_plug_vizia::vizia::{image, prelude::*};
use nih_plug_vizia::{create_vizia_editor, vizia::views::VStack, ViziaState, ViziaTheming};
use views::effect_view::effect_view;
use views::sidebar::sidebar;
use views::top_bar::top_bar;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

#[derive(Lens, Clone, Debug)]
pub(crate) struct Data {
    pub(crate) noise_gate: Arc<AtomicF32>
}

impl Model for Data {}

pub(crate) fn create(
    editor_data: Data,
    chain_ptr: ChainHandle,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_stylesheet(include_str!("./editor.css")).unwrap();
        editor_data.clone().build(cx);
        chain_ptr.clone().build(cx);

        top_bar(cx);

        HStack::new(cx, |cx| {
            sidebar(cx);
            effect_view(cx);
        })
        .class("bottom-row");
    })
}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}
