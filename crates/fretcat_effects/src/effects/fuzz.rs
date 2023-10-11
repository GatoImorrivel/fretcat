use fretcat_macros::{Message, getter};
use serde::{Serialize, Deserialize};
use fretcat_common::vizia::prelude::*;

use crate::Chain;
use crate::ChainData;

use super::AudioEffect;

#[derive(Debug, Message,Clone, Copy, Serialize, Deserialize)]
pub struct Fuzz {
    #[msg]
    pub volume: f32
}

impl Default for Fuzz {
    fn default() -> Self {
        Self {
            volume: 1.0,
        }
    }
}

impl AudioEffect for Fuzz {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32])) {
    }

    fn view(&self, cx: &mut Context, effect: usize) {
        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(volume), false)
                    .on_changing(|cx, val| cx.emit(Message::Volume(val)))
                    .class("volume-knob");
                Label::new(cx, "Output Gain");
            })
            .class("fuzz-knob-group");
        })
        .class("fuzz");
    }

    fn update(&mut self, event: &mut Event) -> Option<()>{
        event.map(|event, _| match event {
            Message::Volume(val) => {
                self.volume = *val;
            }
        });

        Some(())
    }

    fn height(&self) -> f32 {
        100.0
    }
}
