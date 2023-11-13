use std::f32::consts::PI;

use crate::prelude::*;

use crate::common::{AudioFilter, FilterMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overdrive {
    pub gain: f32,
    pub freq: f32,
    pub volume: f32,
    max_freq_hz: f32,
    min_freq_hz: f32,
    filter: [AudioFilter; NUM_CHANNELS],
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
            freq: min_freq_hz,
            volume: 1.0,
            max_freq_hz: 2000.0,
            min_freq_hz,
            filter: [AudioFilter::new(FilterMode::Lowpass, 44100.0, min_freq_hz, 1.0); 2],
        }
    }
}

impl AudioEffect for Overdrive {
    fn process(&mut self, input_buffer: &mut Frame, _transport: &nih_plug::prelude::Transport) {
        input_buffer.process_individual(|left, right| {
            let clipping_fn = |sample: f32| (2.0 / PI) * f32::atan(sample);

            let output_gain = db_to_gain_fast(self.volume);

            *left = clipping_fn(*left * db_to_gain_fast(self.gain)) * output_gain;
            *right = clipping_fn(*right * db_to_gain_fast(self.gain)) * output_gain;

            *left = self.filter[0].tick(*left);
            *right = self.filter[1].tick(*right);
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        OverdriveView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
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
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Gain", Self::gain, false, 1.0..20.0)
                    .on_changing(|ex, val| ex.emit(Message::Gain(val)))
                    .class("gain-knob");
                NamedKnob::new(
                    cx,
                    "Tone",
                    Self::freq,
                    false,
                    handle.min_freq_hz..handle.max_freq_hz,
                )
                .on_changing(|ex, val| ex.emit(Message::Freq(val)))
                .class("tone-knob");
                NamedKnob::new(cx, "Output Volume", Self::volume, false, -10.0..10.0)
                    .on_changing(|ex, val| ex.emit(Message::Volume(val)))
                    .class("volume-knob");
                Label::new(cx, "Drive").class("effect-title");
            });
        })
    }
}

impl View for OverdriveView {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
                self.handle.gain = *val;
            }
            Message::Freq(val) => {
                self.freq = *val;
                self.handle.freq = *val;
                self.handle.filter.iter_mut().for_each(|filter| {
                    filter.set_cutoff(
                        *val,
                    );
                });
            }
            Message::Volume(val) => {
                self.volume = *val;
                self.handle.volume = *val;
            }
        });
    }
}
