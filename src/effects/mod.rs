use std::fmt::Debug;

use nih_plug::nih_log;


use self::ui::{EffectUI, OverdriveUI};

pub mod ui;
pub mod chain;

#[macro_use]
mod macros;

messages!(
    OverdriveMessage
);

#[derive(Debug, Clone, Copy)]
pub struct EffectUpdate {
    id: usize,
    message: EffectMessage
}

impl EffectUpdate {
    pub fn take(self) -> (usize, EffectMessage) {
        (self.id, self.message)
    }
}

pub trait Effect: Debug {
    fn process(&self, sample: f32) -> f32;
    fn update(&mut self, message: EffectMessage);
    fn ui(&self, id: usize) -> Box<dyn EffectUI + Send + Sync>;
}

#[derive(Debug, Clone, Copy)]
pub enum OverdriveMessage {
    Gain(f32)
}

#[derive(Debug, Default)]
pub struct OverdriveEffect {
    gain: f32
}

impl Effect for OverdriveEffect {
    fn process(&self, sample: f32) -> f32 {
        self.gain * sample
    }

    fn update(&mut self, message: EffectMessage) {
        match message {
            EffectMessage::OverdriveMessage(msg) => {
                match msg {
                    OverdriveMessage::Gain(gain) => self.gain = gain
                }
            },
            _ => nih_log!("Overdrive received invalid message, discarding")
        }
    }

    fn ui(&self, id: usize) -> Box<dyn EffectUI + Send + Sync> {
        let effect = OverdriveUI::new(id, self.gain);

        Box::new(effect)
    }
}