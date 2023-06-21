use std::{fmt::format, ops::Range};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::{
    chain::ChainHandle,
    effects::{Effect, EffectHandle},
};

use super::EDITOR_HEIGHT;

#[derive(Debug, Clone, Copy, Lens)]
struct EffectViewData {
    list_len: usize,
}

enum EffectViewEvent {
    SetScrollY(f32),
}

impl Model for EffectViewData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            EffectViewEvent::SetScrollY(new_y) => {
                let current = cx.current();
                let dpi = cx.scale_factor();
                let container_height = cx.cache.get_height(current) / dpi;
                nih_log!("{}", container_height);
            }
        });
    }
}

pub fn effect_view(cx: &mut Context) {
    EffectViewData {
        list_len: ChainHandle::effects.get(cx).len(),
    }
    .build(cx);

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
        .height(Pixels(EDITOR_HEIGHT as f32 * 0.95))
        .on_scroll(|cx, _, y| {
            cx.emit(EffectViewEvent::SetScrollY(y));
        });
    });
}

fn calculate_visible_list(effects: &Vec<EffectHandle>) {
    let mut effects_height = 0.0;
    let mut list_len = 0usize;
    for effect in effects.iter() {
        if effects_height + effect.height() < EDITOR_HEIGHT as f32 * 0.95 {
            effects_height += effect.height();
            list_len += 1;
        }
    }
}
