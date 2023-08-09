use std::f32::consts::PI;

use nih_plug::wrapper::clap::lazy_static;
use nih_plug_vizia::vizia::prelude::*;

use crate::{chain::ChainHandle, effect::AudioEffect, ChainData, Effect};

use effects_derive::Getters;

#[derive(Debug, Clone, Copy, Getters)]
pub struct Overdrive {
    #[get]
    pub gain: f32,
    #[get]
    pub blend: f32,
    pub threshold: f32,
    pub volume: f32,
}

impl Default for Overdrive {
    fn default() -> Self {
        Self {
            gain: 1.0,
            blend: 1.0,
            threshold: 1.0,
            volume: 1.0,
        }
    }
}

enum Message {
    Gain(f32),
    Threshold(f32),
}

impl AudioEffect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let clean = _sample;
        let threshold = self.threshold * 100.0;
        let amplified = _sample * self.gain * threshold;
        let distorted = (2.0 / PI) * f32::atan(amplified);

        let output_gain = self.volume * 10.0;

        ((distorted * self.blend) + (clean * (1.0 - self.blend))) * output_gain
    }

    fn view(&self, cx: &mut Context, effect: Effect, chain: ChainHandle) {
        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, (get_gain)(effect.clone()), false)
                    .on_changing(|cx, val| cx.emit(Message::Gain(val)));
                Label::new(cx, "Gain");
            })
            .class("overdrive-knob-group");
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, OverdriveControl::threshold, false)
                    .on_changing(|cx, val| cx.emit(Message::Threshold(val)));
                Label::new(cx, "Threshold");
            })
            .class("overdrive-knob-group");
        });
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: ChainHandle) {
        let data = unsafe {
            chain
                .as_ptr()
                .as_mut()
                .unwrap()
                .query_cast_mut::<Self>(&effect)
                .unwrap()
        };

        event.map(|event, _| match event {
            Message::Gain(val) => {
                data.gain = *val;
            }
            Message::Threshold(val) => {
                data.threshold = *val;
            }
        });
    }
}