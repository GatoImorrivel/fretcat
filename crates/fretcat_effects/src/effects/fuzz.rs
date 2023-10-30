use std::sync::Arc;

use fretcat_macros::{Message};
use nih_plug::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{arc_to_mut};

use super::AudioEffect;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Fuzz {
    pub volume: f32,
}

impl Default for Fuzz {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}

impl AudioEffect for Fuzz {
    fn process(&mut self, _input_buffer: (&mut [f32], &mut [f32])) {}

    fn view(&self, cx: &mut Context, effect: Arc<dyn AudioEffect>) {
        FuzzView::new(cx, effect.into_any_arc().downcast::<Self>().unwrap());
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Data, Message, Lens)]
pub struct FuzzView {
    #[msg]
    pub volume: f32,

    #[lens(ignore)]
    #[data(ignore)]
    effect: Arc<Fuzz>,
}

impl FuzzView {
    pub fn new(cx: &mut Context, effect: Arc<Fuzz>) -> Handle<Self> {
        Self {
            volume: effect.volume,
            effect,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::volume, false)
                        .on_changing(|cx, val| cx.emit(Message::Volume(val)))
                        .class("volume-knob");
                    Label::new(cx, "Output Gain");
                })
                .class("fuzz-knob-group");
            })
            .class("fuzz");
        })
    }
}

impl View for FuzzView {
    fn element(&self) -> Option<&'static str> {
        Some("fuzz")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| {
            let effect = unsafe { arc_to_mut(&self.effect) };
            match event {
                Message::Volume(val) => {
                    self.volume = *val;
                    effect.volume = *val;
                }
            }
        });
    }
}
