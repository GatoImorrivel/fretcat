use std::{cell::Cell, sync::Arc};

use fretcat_effects::chain::ChainHandle;
use nih_plug::prelude::*;
use nih_plug_vizia::{create_vizia_editor, vizia::prelude::*, ViziaState};

use crate::{
    card::{Card, CardData},
    effect_view::effect_view,
    sidebar::sidebar,
    top_bar::top_bar,
    EDITOR_HEIGHT, EDITOR_WIDTH,
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
            }
            .build(cx);
            cx.add_stylesheet(include_str!("../css/cards.css")).unwrap();

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    top_bar(cx);
                })
                .class("topbar-wrapper");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        sidebar(cx);
                    })
                    .class("sidebar-wrapper");
                    VStack::new(cx, |cx| {
                        effect_view(cx);
                    })
                    .class("list-wrapper");
                })
                .class("content-wrapper");
            })
            .class("main");

            Binding::new(cx, CardData::dragging, |cx, bind| {
                let dragging = bind.get(cx);
                if let Some(dragging) = dragging {
                    Binding::new(cx, CardData::cursor, move |cx, bind| {
                        let cursor = bind.get(cx);
                        VStack::new(cx, |cx| {
                            (dragging.content)(cx);
                        })
                        .class("card-base")
                        .width(Pixels(300.0))
                        .position_type(PositionType::SelfDirected)
                        .left(Pixels(cursor.0))
                        .top(Pixels(cursor.1));
                    });
                }
            });
        },
    )
}
