use std::{f32::consts::PI, fmt::Debug};

use fretcat_macros::{getter, Message};
use fretcat_common::nih_plug::util::db_to_gain_fast;
use fretcat_common::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Chain, ChainData};

use crate::common::{Filter, FilterMode, map_normalized_value};

use super::AudioEffect;

#[derive(Debug, Clone, Message, Serialize, Deserialize)]
pub struct Overdrive {
    #[msg]
    pub gain: f32,
    #[msg]
    pub freq: f32,
    #[msg]
    pub volume: f32,
    max_freq_hz: f32,
    min_freq_hz: f32,
    filter: Filter
}

impl Default for Overdrive {
    fn default() -> Self {
        let min_freq_hz = 1000.0;
        Self {
            gain: 1.0,
            freq: 0.0,
            volume: 1.0,
            max_freq_hz: 2000.0,
            min_freq_hz,
            filter: Filter::new(FilterMode::Lowpass, 44100.0, min_freq_hz, 1.0)
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
            *sample = self.filter.tick(*sample);
        });
    }

    fn view(&self, cx: &mut Context, effect: usize) {
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

    fn update(&self, event: &mut Event, effect: usize, chain: &mut Chain) -> Option<()> {
        let data = chain.query_cast_mut::<Self>(effect)?;
        event.map(|event, _| match event {
            Message::Gain(val) => {
                data.gain = *val;
            }
            Message::Freq(val) => {
                data.freq = *val;
                data.filter.recalculate_coeffs(map_normalized_value(*val, self.min_freq_hz, self.max_freq_hz), self.filter.q());
                fretcat_common::nih_plug::nih_log!("{:#?}", data.filter);
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
