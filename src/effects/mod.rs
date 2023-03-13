mod macros;

use fundsp::{prelude::{BiquadCoefs, Fir}, hacker32::fir};
use nih_plug::buffer::ChannelSamples;
use nih_plug_iced::{Element, Text};

use crate::create_effects;

#[derive(Debug, Clone, Copy)]
pub enum EffectCategory {
    Distortion,
    Filter,
}

#[derive(Clone)]
pub struct Effect {
    name: String,
    kind: EffectCategory,
    state: EffectState,
    algorithm: fn(&EffectState, f32, &ChannelSamples) -> f32,
}

impl Effect {
    pub fn info(&self) -> (&String, &EffectCategory) {
        (&self.name, &self.kind)
    }

    pub fn process(&self, sample: f32, channel_samples: &ChannelSamples) -> f32 {
        (self.algorithm)(&self.state, sample, channel_samples)
    }
}

create_effects!(
    Overdrive {
        gain: f32
    },
    Lowpass {
        freq: f32
    }
);

pub fn make_overdrive() -> Effect {
    Effect {
        name: "Overdrive".to_owned(),
        kind: EffectCategory::Distortion,
        state: EffectState::Overdrive { gain: 2.0 },
        algorithm: |state, sample, channel| match state {
            EffectState::Overdrive { gain } => sample * gain,
            _ => unreachable!(),
        },
    }
}

pub fn make_lowpass() -> Effect {
    let freq = 440.0;
    let coefs = BiquadCoefs::butter_lowpass(44100.0, freq);
    Effect {
        name: "Lowpass".to_owned(),
        kind: EffectCategory::Filter,
        state: EffectState::Lowpass {
            freq: freq,
        },
        algorithm: |state, sample, channel_samples| match state {
            EffectState::Lowpass { 
                freq, 
            } => {
                sample
            },
            _ => unreachable!(),
        },
    }
}
