use std::sync::Arc;
use std::{f32::consts::PI, fmt::Debug};

use fretcat_macros::Message;
use nih_plug::util::db_to_gain_fast;
use nih_plug::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{NUM_CHANNELS, EffectHandle};

use crate::common::{map_normalized_value, Filter, FilterMode};

use super::AudioEffect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overdrive {
    pub gain: f32,
    pub freq: f32,
    pub volume: f32,
    max_freq_hz: f32,
    min_freq_hz: f32,
    filter: [Filter; NUM_CHANNELS],
}

impl PartialEq for Overdrive {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain && self.freq == other.freq && self.volume == other.volume
    }
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
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
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

    fn view(&self, cx: &mut Context, effect: Arc<dyn AudioEffect>) {
        OverdriveView::new(cx, EffectHandle::<Self>::from(effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Data, Lens, Message)]
pub struct OverdriveView {
    #[msg]
    pub gain: f32,
    #[msg]
    pub freq: f32,
    #[msg]
    pub volume: f32,

    #[lens(ignore)]
    #[data(ignore)]
    handle: EffectHandle<Overdrive>,
}

impl OverdriveView {
    pub fn new(cx: &mut Context, handle: EffectHandle<Overdrive>) -> Handle<Self> {
        Self {
            gain: handle.gain,
            freq: handle.freq,
            volume: handle.volume,
            handle,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::gain, false)
                        .on_changing(|cx, val| cx.emit(Message::Gain(val)))
                        .class("gain-knob");
                    Label::new(cx, "Gain");
                })
                .class("knob-group");
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::freq, false)
                        .on_changing(|cx, val| cx.emit(Message::Freq(val)))
                        .class("tone-knob");
                    Label::new(cx, "Tone");
                })
                .class("knob-group");
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::volume, false)
                        .on_changing(|cx, val| cx.emit(Message::Volume(val)))
                        .class("volume-knob");
                    Label::new(cx, "Output Gain");
                })
                .class("knob-group");
            });
        })
    }
}

impl View for OverdriveView {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| {
            match event {
                Message::Gain(val) => {
                    self.gain = *val;
                    self.handle.gain = *val;
                }
                Message::Freq(val) => {
                    self.freq = *val;
                    self.handle.freq = *val;
                    let min_freq = self.handle.min_freq_hz;
                    let max_freq = self.handle.max_freq_hz;
                    self.handle.filter.iter_mut().for_each(|filter| {
                        filter.recalculate_coeffs(
                            map_normalized_value(*val, min_freq, max_freq),
                            filter.q(),
                        );
                    });
                }
                Message::Volume(val) => {
                    self.volume = *val;
                    self.handle.volume = *val;
                }
            }
        });
    }
}
