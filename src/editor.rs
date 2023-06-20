use std::ops::DerefMut;
use std::sync::Arc;

use nih_plug::nih_log;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{create_vizia_editor, vizia::views::VStack, ViziaState, ViziaTheming};

use crate::chain::{ChainPtr, Chain};
use crate::params::FretcatParams;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[derive(Lens, Clone, Debug)]
pub(crate) struct Data {
    pub(crate) params: Arc<FretcatParams>,
    pub(crate) chain_ptr: ChainPtr,
}

impl Model for Data {}

pub(crate) fn create(editor_data: Data, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_stylesheet(include_str!("./default.css")).unwrap();
        editor_data.clone().build(cx);

        VStack::new(cx, |cx| {
            // Top bar
            HStack::new(cx, |cx| {
                Label::new(cx, "Bolas");
            })
            .class("top-bar");

            // Bottom Row
            HStack::new(cx, |cx| {
                // Sidebar
                VStack::new(cx, |cx| {
                    Label::new(cx, "Bolas");
                })
                .class("sidebar");

                // Effect List
                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    ChainPtr::render(cx, Data::chain_ptr);
                })
                .height(Pixels(700.0))
                .class("effect-view");
            })
            .class("bottom-row");
        });
    })
}
