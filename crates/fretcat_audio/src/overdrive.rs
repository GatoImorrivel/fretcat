use std::f32::consts::PI;

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::effect::Effect;


#[derive(Debug, Clone, Copy, Default)]
pub struct Overdrive {
    gain: f32,
    blend: f32,
    threshold: f32,
    volume: f32,
}

impl Effect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
        let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

        blend * self.volume
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
