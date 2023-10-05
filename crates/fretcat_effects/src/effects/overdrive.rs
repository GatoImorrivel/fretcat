use std::{f32::consts::PI, fmt::Debug};

use fretcat_macros::{getter, Message};
use nih_plug::util::db_to_gain_fast;
use nih_plug_vizia::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Chain, ChainData, common::Highpass};

use super::{AudioEffect, Effect};

#[derive(Debug, Clone, Message, Serialize, Deserialize)]
pub struct Overdrive {
    #[msg]
    pub gain: f32,
    #[msg]
    pub freq: f32,
    #[msg]
    pub volume: f32,
}

impl Default for Overdrive {
    fn default() -> Self {
        Self {
            gain: 1.0,
            freq: 0.0,
            volume: 1.0,
        }
    }
}

impl AudioEffect for Overdrive {
    fn process(&self, input_buffer: &mut [f32]) {
        input_buffer.iter_mut().for_each(|sample| {
            let clean = *sample;
            let amplified = *sample * db_to_gain_fast(self.gain * 10.0);
            let distorted = (2.0 / PI) * f32::atan(amplified);

            let output_gain = db_to_gain_fast(self.volume * 10.0);

            *sample = ((distorted * self.gain) + (clean * (1.0 - self.gain))) * output_gain;
        });
    }

    fn view(&self, cx: &mut Context, effect: Effect) {
        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(gain), false)
                    .on_changing(|cx, val| cx.emit(Message::Gain(val)))
                    .class("gain-knob");
                Label::new(cx, "Gain");
            })
            .class("overdrive-knob-group");
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(freq), false)
                    .on_changing(|cx, val| cx.emit(Message::Freq(val)))
                    .class("tone-knob");
                Label::new(cx, "Tone");
            })
            .class("overdrive-knob-group");
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(volume), false)
                    .on_changing(|cx, val| cx.emit(Message::Volume(val)))
                    .class("volume-knob");
                Label::new(cx, "Output Gain");
            })
            .class("overdrive-knob-group");
        })
        .class("overdrive");
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()> {
        let data = chain.query_cast_mut::<Self>(&effect)?;
        event.map(|event, _| match event {
            Message::Gain(val) => {
                data.gain = *val;
            }
            Message::Freq(val) => {
                data.freq = *val;
            }
            Message::Volume(val) => {
                data.volume = *val;
            }
        });

        Some(())
    }

    fn height(&self) -> f32 {
        100.0
    }
}
