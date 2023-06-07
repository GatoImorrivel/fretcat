use core::fmt;
use std::f32::consts::PI;

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect: fmt::Debug {
    fn process(&self, _sample: f32) -> f32;
    fn ui(&mut self, cx: &mut Context);
}

#[derive(Debug, Clone, Copy)]
pub struct Overdrive {
    gain: f32,
    blend: f32,
    threshold: f32,
    volume: f32
}

impl Default for Overdrive {
    fn default() -> Self {
        Self {
            gain: 10.0,
            blend: 0.1,
            threshold: 50.0,
            volume: 1.0
        }
    }
}

impl Effect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
        let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

        blend * self.volume
    }

    fn ui(&mut self, cx: &mut Context) {
        HStack::new(cx, |cx| {
            Label::new(cx, &format!("Gain: {0}", self.gain))
            .color(Color::white());
        });
    }
}