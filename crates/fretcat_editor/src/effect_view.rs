use fretcat_effects::chain::{ChainHandle, ChainEvent};
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

use crate::card::{CardData, CardEvent};

const EFFECT_BAR_HEIGHT: f32 = 0.0;

pub fn effect_view(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/list.css")).unwrap();
    VStack::new(cx, |cx| {}).class("preset-control");

    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
        Binding::new(cx, ChainHandle::redraw, |cx, _| {
        let chain = ChainHandle::root.get(cx);

        for (i, effect) in chain.effects.iter().enumerate() {
            let data = chain.query(effect).unwrap();
            VStack::new(cx, |cx| {
                data.view(cx, effect);
            })
                .on_drop(move |ex, _| {
                    let index = calculate_effect_index(i, ex.mouse(), ex.bounds());

                    let card = CardData::dragging.get(ex);

                    if let Some(card) = card {
                        ex.emit(ChainEvent::Insert(card.spawn(), index));
                        ex.emit(CardEvent::DragChange(None));
                    }
                })
                .width(Percentage(100.0))
                .height(Pixels(data.height()));
        }
        })
    })
    .class("list");
}

fn calculate_effect_index(i: usize, mouse: &MouseState<Entity>, bounds: BoundingBox) -> usize {
    let middle_point = (bounds.y + bounds.h) / 2.0;

    if mouse.cursory < middle_point {
        if !i <= 0 {
            i - 1
        } else {
            i
        }
    } else {
        i + 1
    }
}
