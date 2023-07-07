use std::sync::Arc;

use fretcat_effects::{
    chain::{ChainEvent, ChainHandle},
    effect::{Effect, self},
};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::card::CardData;

pub fn effect_view(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
            Binding::new(cx, ChainHandle::effects, |cx, effects| {
                let mut effects = effects.get(cx);

                for (i, effect) in effects.iter_mut().enumerate() {
                    VStack::new(cx, |cx| {
                        let handle = effect.clone();
                        effect.view(cx, handle);
                        Label::new(cx, &i.to_string());
                    })
                    .width(Percentage(100.0))
                    .on_drop(move |ex, _| {
                        let mouse = ex.mouse();
                        let bounds = ex.bounds();

                        let middle_point = (bounds.y + bounds.h) / 2.0;

                        let index = if mouse.cursory < middle_point {
                            if !i <= 0 {
                                i - 1
                            } else {
                                i
                            }
                        } else {
                            i + 1
                        };

                        nih_log!("{}", index);

                        let wrapper = CardData::dragging.get(ex);
                        let effect = wrapper.take();

                        nih_log!("DROPPED: {:?}", effect);

                        if let Some(effect) = effect {
                            ex.emit(ChainEvent::InsertEffect(index, effect));
                        }
                    })
                    .height(Pixels(effect.height()));
                }
            });
        });
    })
    .class("list");
}
