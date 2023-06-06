use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{
    create_vizia_editor,
    vizia::{prelude::Lens, state::Model, views::VStack},
    widgets::ResizeHandle,
    ViziaState, ViziaTheming,
};

use crate::chain::ChainPtr;
use crate::params::FretcatParams;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

#[derive(Lens, Clone)]
pub(crate) struct Data {
    pub(crate) params: Arc<FretcatParams>,
    pub(crate) chain: ChainPtr,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

pub(crate) fn create(editor_data: Data, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        editor_data.clone().build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "Bolas");
            })
            .background_color(Color::black());

            HStack::new(cx, |cx| {
                Label::new(cx, "Bolas");
            });
        });
    })
}
