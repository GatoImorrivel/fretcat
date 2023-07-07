use std::sync::Arc;

use fretcat_effects::{chain::{ChainHandle, ChainEvent}, effect::Effect};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::card::CardData;

pub fn effect_view(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
            Binding::new(cx, ChainHandle::effects, |cx, effects| {
                let mut effects = effects.get(cx);

                for effect in effects.iter_mut() {
                    VStack::new(cx, |cx| {
                        let handle = effect.clone();
                        effect.view(cx, handle);
                    })
                    .width(Percentage(100.0))
                    .height(Pixels(effect.height()));
                }
            });
        })
        .on_drop(|ex, _| {
            let wrapper = CardData::dragging.get(ex);
            let effect = wrapper.take();

            nih_log!("DROPPED: {:?}", effect);

            if let Some(effect) = effect {
                ex.emit(ChainEvent::PushEffect(effect));
            }
        });
    })
    .class("list");
}
