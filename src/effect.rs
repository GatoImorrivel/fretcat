use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect {
    fn process(&self, _buffer: &[f32], _sample: f32) -> f32;
    fn ui(&mut self, cx: &mut Context);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Overdrive {
    gain: f32,
}

impl Overdrive {
    pub fn gain(&self) -> f32 {
        self.gain
    }
}

impl Effect for Overdrive {
    fn process(&self, _buffer: &[f32], _sample: f32) -> f32 {
        return self.gain * _sample;
    }

    fn ui(&mut self, cx: &mut Context) {
        HStack::new(cx, |cx| {
            Label::new(cx, &format!("Gain: {0}", self.gain))
            .color(Color::white());
        });
    }
}