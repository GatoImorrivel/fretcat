use std::default;

use serde::{Serialize, Deserialize};

pub mod field;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Effects {
    Overdrive(Overdrive)
}

impl Effects {
    pub fn into_inner(&self) -> &impl Effect {
        match self {
            Effects::Overdrive(overdrive) => overdrive
        }
    }
}

pub trait Effect {
    fn process(&self, sample: f32) -> f32;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Overdrive {
    gain: f32,
}

impl Default for Overdrive {
    fn default() -> Self {
        Self {
            gain: 0.0
        }
    }
}

impl Effect for Overdrive {
    fn process(&self, sample: f32) -> f32 {
        self.gain * sample
    }
}

impl Into<Effects> for Overdrive {
    fn into(self) -> Effects {
        Effects::Overdrive(self)
    }
}