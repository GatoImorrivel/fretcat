use std::{sync::Arc};

use fretcat_effects::{chain::ChainHandle, EffectKind};
use nih_plug::prelude::*;
use nih_plug_vizia::{create_vizia_editor, vizia::prelude::*, ViziaState};

use crate::{
    card::{CardData, card_drag},
    effect_view::effect_view,
    sidebar::{sidebar, SidebarData, SidebarTab}, left_bar::left_bar,
};

#[derive(Lens, Clone, Debug)]
pub struct Data {
    pub noise_gate: Arc<AtomicF32>,
}

impl Model for Data {}

pub fn create(
    editor_data: Data,
    chain_handle: ChainHandle,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        editor_state,
        nih_plug_vizia::ViziaTheming::Custom,
        move |cx, _| {
            cx.add_stylesheet(include_str!("../css/editor.css"))
                .unwrap();

            editor_data.clone().build(cx);
            chain_handle.clone().build(cx);

            CardData {
                dragging: None,
                cursor: (0.0, 0.0),
                effect_kinds: EffectKind::variants(),
                selected_kind: 0
            }
            .build(cx);

            SidebarData {
                current_tab: SidebarTab::Component
            }.build(cx);

            cx.add_stylesheet(include_str!("../css/cards.css")).unwrap();

            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    left_bar(cx);
                })
                .class("leftbar-wrapper");
                VStack::new(cx, |cx| {
                    sidebar(cx);
                })
                .class("sidebar-wrapper");
                VStack::new(cx, |cx| {
                    effect_view(cx);
                })
                .class("list-wrapper");
            })
            .class("main");

            card_drag(cx);
        },
    )
}
