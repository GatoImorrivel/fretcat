

use fretcat_effects::{
    chain::{ChainEvent, ChainHandle},
};

use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

use crate::card::{CardData, CardEvent};

const EFFECT_BAR_HEIGHT: f32 = 20.0;

pub fn effect_view(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
            Binding::new(cx, ChainHandle::effects, |cx, effects| {
                let mut effects = effects.get(cx);

                for (i, effect) in effects.iter_mut().enumerate() {
                    VStack::new(cx, |cx| {
                        let handle = effect.clone();
                        HStack::new(cx, |cx| {
                            Label::new(cx, &effect.title());
                            Button::new(
                                cx,
                                move |ex| {
                                    ex.emit(ChainEvent::RemoveEffect(i));
                                },
                                |cx| Label::new(cx, "Delete"),
                            );
                        })
                        .height(Pixels(EFFECT_BAR_HEIGHT));
                        effect.view(cx, handle);
                    })
                    .width(Percentage(100.0))
                    .on_drop(move |ex, _| {
                        let index = calculate_effect_index(i, ex.mouse(), ex.bounds());

                        let card = CardData::dragging.get(ex);

                        if let Some(card) = card {
                            ex.emit(ChainEvent::InsertEffect(index, card.spawn()));
                            ex.emit(CardEvent::DragChange(None));
                        }
                    })
                    .height(Pixels(effect.height() + EFFECT_BAR_HEIGHT));
                }
                Element::new(cx)
                    .height(Stretch(1.0))
                    .width(Percentage(100.0))
                    .on_drop(|ex, _| {
                        let card = CardData::dragging.get(ex);

                        if let Some(card) = card {
                            ex.emit(ChainEvent::PushEffect(card.spawn()));
                            ex.emit(CardEvent::DragChange(None));
                        }
                    });
            });
        });
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
