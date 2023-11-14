use serde::{Deserialize, Serialize};

use crate::{effects::AudioEffect, frame::Frame, NUM_CHANNELS};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct NoiseGate {
    gates: [Gate; NUM_CHANNELS],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Gate {
    threshold_db: f32,
    attack_seconds: f32,
    release_seconds: f32,
    is_gate_open: bool,
    current_gain: f32,
}

impl Default for Gate {
    fn default() -> Self {
        Self {
            attack_seconds: 0.2,
            release_seconds: 0.2,
            threshold_db: -12.0,
            is_gate_open: false,
            current_gain: 0.0,
        }
    }
}

impl NoiseGate {
    pub fn set_attack_seconds(&mut self, seconds: f32) {
        self.gates
            .iter_mut()
            .for_each(|gate| gate.attack_seconds = seconds);
    }

    pub fn set_release_seconds(&mut self, seconds: f32) {
        self.gates
            .iter_mut()
            .for_each(|gate| gate.release_seconds = seconds);
    }

    pub fn set_threshold(&mut self, threshold: f32) {
        self.gates
            .iter_mut()
            .for_each(|gate| gate.threshold_db = threshold);
    }

    fn process_channel(gate: &mut Gate, sample: f32) -> f32 {
        let input_db = sample.abs().log10() * 20.0;

        if input_db >= gate.threshold_db {
            gate.is_gate_open = true;
        } else {
            if gate.is_gate_open {
                gate.current_gain = (1.0 - gate.release_seconds) * gate.current_gain;
            } else {
                gate.current_gain = gate.attack_seconds * (input_db - gate.threshold_db);
            }
        }

        let output = sample * 10.0_f32.powf(gate.current_gain / 20.0);

        output
    }
}

impl AudioEffect for NoiseGate {
    fn process(&mut self, input_buffer: &mut Frame, _transport: &nih_plug::prelude::Transport) {
        input_buffer.process_individual(|left, right| {
            *left = Self::process_channel(&mut self.gates[0], *left);
            *right = Self::process_channel(&mut self.gates[1], *right);
        })
    }
}
