use std::{fmt::format, ops::Range};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::{
    chain::ChainHandle,
    effects::{Effect, EffectHandle},
};

use super::EDITOR_HEIGHT;

pub fn effect_view(cx: &mut Context) {
    Binding::new(cx, ChainHandle::effects, |cx, effects| {
        ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
            let effects = &mut effects.get(cx);
            for i in effects {
                VStack::new(cx, |cx| {
                    i.render(cx);
                })
                .background_color(Color::aliceblue())
                .height(Pixels(i.height()));
            }
        })
        .height(Pixels(EDITOR_HEIGHT as f32 * 0.95));
    });
}
