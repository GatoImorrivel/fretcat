use fretcat_effects::chain::{ChainEvent, ChainHandle};
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

use crate::{
    card::{CardData, CardEvent},
    effect_list::EffectList,
};

const EFFECT_BAR_HEIGHT: f32 = 30.0;

pub fn effect_view(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/list.css")).unwrap();
    VStack::new(cx, |cx| {}).class("preset-control");

    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
        EffectList::new(
            cx,
            EFFECT_BAR_HEIGHT,
            move |cx, i, effect, data| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Button::new(cx, move |ex| ex.emit(ChainEvent::Remove(effect)), |cx| {
                            Label::new(cx, "Deletar")
                        });
                        Button::new(cx, move |ex| ex.emit(ChainEvent::Remove(effect)), |cx| {
                            Label::new(cx, "Disable")
                        });
                    })
                    .width(Percentage(100.0))
                    .height(Pixels(EFFECT_BAR_HEIGHT));
                    data.view(cx, &effect);
                }).needs_redraw();
            },
        )
        .width(Percentage(100.0));
    })
    .class("list");
}


