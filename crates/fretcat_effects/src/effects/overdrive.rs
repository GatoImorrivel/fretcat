use std::{f32::consts::PI, fmt::Debug};

use fretcat_common::nih_plug::util::db_to_gain_fast;
use fretcat_common::vizia::prelude::*;
use fretcat_macros::{getter, Message};
use serde::{Deserialize, Serialize};

use crate::{Chain, ChainData, NUM_CHANNELS};

use crate::common::{map_normalized_value, Filter, FilterMode};

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
    filter: [Filter; NUM_CHANNELS],
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
            filter: [Filter::new(FilterMode::Lowpass, 44100.0, min_freq_hz, 1.0); 2],
        }
    }
}

impl AudioEffect for Overdrive {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32])) {
        input_buffer
            .0
            .iter_mut()
            .zip(input_buffer.1.iter_mut())
            .for_each(|(left, right)| {
                let clipping_fn = |sample: f32| (2.0 / PI) * f32::atan(sample);

                let output_gain = db_to_gain_fast(self.volume * 10.0);

                *left = clipping_fn(*left * db_to_gain_fast(self.gain * 10.0)) * output_gain;
                *right = clipping_fn(*right * db_to_gain_fast(self.gain * 10.0)) * output_gain;

                *left = self.filter[0].tick(*left);
                *right = self.filter[1].tick(*right);
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

    fn update(&mut self, event: &mut Event) -> Option<()> {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
            }
            Message::Freq(val) => {
                self.freq = *val;
                self.filter.iter_mut().for_each(|filter| {
                    filter.recalculate_coeffs(
                        map_normalized_value(*val, self.min_freq_hz, self.max_freq_hz),
                        filter.q(),
                    );
                });
            }
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
