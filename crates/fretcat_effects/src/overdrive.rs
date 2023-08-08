use std::{f32::consts::PI, default};

use crate::effect::AudioEffect;


#[derive(Debug, Clone, Copy)]
pub struct Overdrive {
    pub gain: f32,
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

impl AudioEffect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let clean = _sample;
        let threshold = self.threshold * 100.0;
        let amplified = _sample * self.gain * threshold;
        let distorted = (2.0 / PI) * f32::atan(amplified);

        let output_gain = self.volume * 10.0;

        ((distorted * self.blend) + (clean * (1.0 - self.blend))) * output_gain
    }
}
