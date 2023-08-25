use std::{f32::consts::PI, fmt::Debug};

use fundsp::{hacker32::highpass_hz, prelude::An, svf::{FixedSvf, HighpassMode}};
use serde::{Serialize, Deserialize};
use fretcat_macros::{getter, Message};
use nih_plug_vizia::vizia::prelude::*;

use crate::{ChainData, Chain};

use super::{AudioEffect, Effect};

#[derive(Clone, Message, Serialize, Deserialize)]
pub struct Overdrive {
    #[msg]
    pub blend: f32,
    #[msg]
    pub freq: f32,
    #[msg]
    pub volume: f32,
    #[serde(skip_serializing, skip_deserializing)]
    filter: Filter
}

#[derive(Clone)]
struct Filter {
    pub filter: An<FixedSvf<f32, f32, HighpassMode<f32>>>
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            filter: highpass_hz(0.0, 0.25)
        }
    }
}

impl Debug for Overdrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Overdrive").field("blend", &self.blend).field("freq", &self.freq).field("volume", &self.volume).finish()
    }
}

impl Default for Overdrive {
    fn default() -> Self {
        Self {
            blend: 1.0,
            freq: 0.0,
            volume: 1.0,
            filter: Filter::default() 
        }
    }
}

impl AudioEffect for Overdrive {
    fn process(&self, input_buffer: &mut [f32]) {
        input_buffer.iter_mut().for_each(|sample| {
            let clean = *sample;
            let amplified = *sample * nih_plug::util::db_to_gain_fast(5.0);
            let distorted = (2.0 / PI) * f32::atan(amplified);

            let output_gain = self.volume * 10.0;

            *sample = ((distorted * self.blend) + (clean * (1.0 - self.blend))) * output_gain;
        });
    }

    fn view(&self, cx: &mut Context, effect: Effect) {
        cx.add_stylesheet(include_str!("../../css/overdrive.css")).unwrap();
        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(blend), false)
                    .on_changing(|cx, val| cx.emit(Message::Blend(val)));
                Label::new(cx, "Gain");
            })
            .class("overdrive-knob-group");
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(freq), false)
                    .on_changing(|cx, val| cx.emit(Message::Freq(val)));
                Label::new(cx, "Tone");
            })
            .class("overdrive-knob-group");
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(volume), false)
                    .on_changing(|cx, val| cx.emit(Message::Volume(val)));
                Label::new(cx, "Output Gain");
            })
            .class("overdrive-knob-group");
        }).class("overdrive");
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()> {
        let data = chain.query_cast_mut::<Self>(&effect)?;
        event.map(|event, _| match event {
            Message::Blend(val) => {
                data.blend = *val;
            }
            Message::Freq(val) => {
                data.freq = *val;
                data.filter.filter.set_cutoff(*val);
            }
            Message::Volume(val) => {
                data.volume = *val;
            }
        });

        Some(())
    }

    fn height(&self) -> f32 {
        200.0
    }
}