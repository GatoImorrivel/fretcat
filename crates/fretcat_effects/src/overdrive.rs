use std::f32::consts::PI;

use nih_plug_vizia::vizia::prelude::*;

use crate::{effect::AudioEffect, ChainData, Effect, Chain};

use effects_derive::{getter, Message};

#[derive(Debug, Clone, Copy, Message)]
pub struct Overdrive {
    #[msg]
    pub gain: f32,
    pub blend: f32,
    #[msg]
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

impl AudioEffect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let clean = _sample;
        let threshold = self.threshold * 100.0;
        let amplified = _sample * self.gain * threshold;
        let distorted = (2.0 / PI) * f32::atan(amplified);

        let output_gain = self.volume * 10.0;

        ((distorted * self.blend) + (clean * (1.0 - self.blend))) * output_gain
    }

    fn view(&self, cx: &mut Context, effect: Effect) {
        cx.add_stylesheet(include_str!("../css/overdrive.css")).unwrap();
        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(gain), false)
                    .on_changing(|cx, val| cx.emit(Message::Gain(val)));
                Label::new(cx, "Gain");
            })
            .class("overdrive-knob-group");
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(threshold), false)
                    .on_changing(|cx, val| cx.emit(Message::Threshold(val)));
                Label::new(cx, "Threshold");
            })
            .class("overdrive-knob-group");
        }).class("overdrive");
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()> {
        let data = chain.query_cast_mut::<Self>(&effect)?;
        event.map(|event, _| match event {
            Message::Gain(val) => {
                data.gain = *val;
            }
            Message::Threshold(val) => {
                data.threshold = *val;
            }
        });

        Some(())
    }

    fn height(&self) -> f32 {
        200.0
    }
}