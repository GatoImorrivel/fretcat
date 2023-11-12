use serde::{Deserialize, Serialize};

use crate::{common::Envelope, effects::AudioEffect, frame::Frame};


#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct NoiseGate {
    threshold: f32,
    adsr: [Envelope; 2]
}

impl NoiseGate {
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
}

impl AudioEffect for NoiseGate {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {

    }
}