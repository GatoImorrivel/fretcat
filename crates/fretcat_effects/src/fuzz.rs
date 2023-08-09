use std::{f32::consts::PI, sync::Arc};

use atomic_refcell::AtomicRefCell;

use nih_plug_vizia::vizia::prelude::*;

use crate::{effect::AudioEffect, Chain, Effect, chain::ChainHandle};
#[derive(Debug, Clone, Copy)]
pub struct Fuzz {
    gain: f32,
    blend: f32,
    threshold: f32,
    volume: f32,
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
    fn process(&self, _sample: f32) -> f32 {
        let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
        let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

        blend * self.volume
    }

    fn view(&self, cx: &mut Context, effect: Effect, chain: ChainHandle) {
        
    }

    fn update(&self, event: &mut Event, effect: Effect, chain: ChainHandle) {
        
    }
}
