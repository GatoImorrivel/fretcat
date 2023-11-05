// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct StageConfig {
    samples: f32,
    duration_secs: f32,
}

impl Default for StageConfig {
    fn default() -> Self {
        StageConfig::new(0.0, Duration::from_secs_f32(0.0))
    }
}

impl StageConfig {
    fn new(samples: f32, duration: Duration) -> Self {
        StageConfig {
            samples: samples.into(),
            duration_secs: duration.as_secs_f32().into(),
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.samples = samples_for_duration(sample_rate, self.duration_secs);
    }

    fn set_duration(&mut self, sample_rate: f32, duration: Duration) {
        self.duration_secs = duration.as_secs_f32();
        self.samples = samples_for_duration(sample_rate, self.duration_secs);
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct EnvelopeConfig {
    attack: StageConfig,
    attack_level: f32,
    decay: StageConfig,
    sustain: f32,
    release: StageConfig,
    sample_rate: f32,
    is_exp: bool,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        EnvelopeConfig {
            attack: StageConfig::new(0.0, Duration::from_secs_f32(0.2)),
            attack_level: 1.0.into(),
            decay: StageConfig::new(0.0, Duration::from_secs_f32(0.3)),
            sustain: 0.8.into(),
            release: StageConfig::new(0.0, Duration::from_secs_f32(0.1)),
            sample_rate: 0.0.into(),
            is_exp: false,
        }
    }
}

impl EnvelopeConfig {
    fn exp() -> Self {
        Self {
            is_exp: true,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct EnvelopeState {
    current_samples: f32,
    stage_start_volume: f32,
    current_volume: f32,
}

impl Default for EnvelopeState {
    fn default() -> Self {
        EnvelopeState {
            current_samples: 0.0.into(),
            stage_start_volume: 0.0.into(),
            current_volume: 0.0.into(),
        }
    }
}

/// An ADSR envelope implementation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Envelope {
    stage: EnvelopeStage,
    state: EnvelopeState,
    config: EnvelopeConfig,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope::new()
    }
}

impl Envelope {
    /// Create a linear envelope with default configuration
    pub fn new() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle.into(),
            state: EnvelopeState::default(),
            config: EnvelopeConfig::default(),
        }
    }

    /// Create an exponential envelope with default configuration
    pub fn exp() -> Self {
        Envelope {
            stage: EnvelopeStage::Idle.into(),
            state: EnvelopeState::default(),
            config: EnvelopeConfig::exp(),
        }
    }

    /// Set the envelope sample rate, required before playback
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.config.sample_rate = sample_rate;
        self.config.attack.set_sample_rate(sample_rate);
        self.config.decay.set_sample_rate(sample_rate);
        self.config.release.set_sample_rate(sample_rate);
    }

    /// Set the envelope sample rate, required before playback
    pub fn set_attack(&mut self, duration: Duration) {
        self.config
            .attack
            .set_duration(self.config.sample_rate, duration);
    }

    /// Set the envelope decay time
    pub fn set_decay(&mut self, duration: Duration) {
        self.config
            .decay
            .set_duration(self.config.sample_rate, duration);
    }

    /// Set the envelope sustain time
    pub fn set_sustain(&mut self, sustain: f32) {
        self.config.sustain = sustain;
    }

    /// Set the envelope release time
    pub fn set_release(&mut self, duration: Duration) {
        self.config
            .release
            .set_duration(self.config.sample_rate, duration);
    }

    /// Get the current volume multiplier
    pub fn volume(&mut self) -> f32 {
        self.update_stage(self.state.current_samples, true);
        self.state.current_volume
    }

    /// Update the envelope, pushing its state forwards by 1 sample
    pub fn tick(&mut self) {
        let current_samples = self.state.current_samples + 1.0;
        self.state.current_samples = current_samples;
        self.update_stage(current_samples, false);
    }

    fn update_stage(&mut self, current_samples: f32, recurse: bool) {
        // println!("update_stage(current_samples={})", current_samples);
        let maybe_stage_config = match self.stage {
            EnvelopeStage::Idle => None,
            EnvelopeStage::Attack => {
                self.state.current_volume =
                    self.calculate_volume(self.config.attack_level, self.config.attack.samples);
                Some(&self.config.attack)
            }
            EnvelopeStage::Decay => {
                self.state.current_volume =
                    self.calculate_volume(self.config.sustain, self.config.decay.samples);
                Some(&self.config.decay)
            }
            EnvelopeStage::Sustain => {
                self.state.current_volume = self.config.sustain;
                None
            }
            EnvelopeStage::Release => {
                self.state.current_volume = self.calculate_volume(0.0, self.config.release.samples);
                Some(&self.config.release)
            }
        };

        if let Some(stage_config) = maybe_stage_config {
            if current_samples >= stage_config.samples {
                self.next_stage();

                // Purpose is to handle 0 value envelopes
                if recurse {
                    self.update_stage(current_samples, true);
                }
            }
        }
    }

    /// Trigger the envelope by setting its stage to the Attack phase. Does not change the current
    /// volume, only the stage.
    pub fn note_on(&mut self) {
        self.set_stage(EnvelopeStage::Attack);
    }

    /// Set the envelope stage to release.
    pub fn note_off(&mut self) {
        self.set_stage(EnvelopeStage::Release);
    }

    fn next_stage(&mut self) {
        match self.stage {
            EnvelopeStage::Attack => {
                self.state.current_volume = self.config.attack_level;
                self.set_stage(EnvelopeStage::Decay);
            }
            EnvelopeStage::Decay => {
                self.set_stage(EnvelopeStage::Sustain);
            }
            EnvelopeStage::Sustain => {
                self.set_stage(EnvelopeStage::Release);
            }
            EnvelopeStage::Release => {
                self.set_stage(EnvelopeStage::Idle);
            }
            EnvelopeStage::Idle => {}
        }
    }

    fn set_stage(&mut self, stage: EnvelopeStage) {
        self.state.stage_start_volume = self.state.current_volume;
        self.state.current_samples = 0.0;
        self.stage = stage;
    }

    fn calculate_volume(&self, target: f32, duration_samples: f32) -> f32 {
        let start = self.state.stage_start_volume;
        let current_samples = self.state.current_samples;

        if self.config.is_exp {
            let current_volume = self.state.current_volume;
            let a = std::f32::consts::E.powf(-1.0 / (duration_samples.max(f32::EPSILON) * 0.3));
            return a * current_volume + (1.0 - a) * target;
        }

        let perc = current_samples / duration_samples.max(f32::EPSILON);
        let diff = target - start;
        start + perc * diff
    }
}

fn samples_for_duration(sample_rate: f32, duration_secs: f32) -> f32 {
    sample_rate * duration_secs
}
