use serde::{Serialize, Deserialize};
use nih_plug_vizia::vizia::prelude::*;

use crate::Chain;

use super::{AudioEffect, Effect};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Fuzz {
    pub gain: f32,
    pub blend: f32,
    pub threshold: f32,
    pub volume: f32,
}

impl Default for Fuzz {
    fn default() -> Self {
        Self {
            gain: 1.0,
            blend: 1.0,
            threshold: 1.0,
            volume: 1.0,
        }
    }
}

impl AudioEffect for Fuzz {
    fn process(&self, input_buffer: &mut [f32]) {
    }

    fn view(&self, cx: &mut Context, effect: Effect) {
        
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()>{
       Some(())
    }

    fn height(&self) -> f32 {
        400.0
    }
}
