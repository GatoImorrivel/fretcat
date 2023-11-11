use std::sync::Arc;

use fretcat_macros::{Message};
use nih_plug::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{EffectHandle, effects::AudioEffect};

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
    fn process(&mut self, _input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {}

    fn view(&self, cx: &mut Context, effect: Arc<dyn AudioEffect>) {
        FuzzView::new(cx, EffectHandle::<Self>::from(effect));
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
    handle: EffectHandle<Fuzz>,
}

impl FuzzView {
    pub fn new(cx: &mut Context, handle: EffectHandle<Fuzz>) -> Handle<Self> {
        Self {
            volume: handle.volume,
            handle,
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
            match event {
                Message::Volume(val) => {
                    self.volume = *val;
                    self.handle.volume = *val;
                }
            }
        });
    }
}
