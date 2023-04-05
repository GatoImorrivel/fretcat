use crate::editor::components::EffectMarker;

pub trait Effect {
    fn process(&self, sample: f32) -> f32;
    fn update(&mut self, message: dyn EffectMarker);
}

pub struct GenericEffectUpdate(usize, Box<dyn EffectMarker + Send + Sync>);

impl GenericEffectUpdate {
    pub fn get_id(&self) -> usize {
        self.0
    }

    pub fn get_message(&self) -> Box<dyn EffectMarker + Send + Sync> {
        self.1
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Overdrive {
    gain: f32,
}

impl Overdrive {
    pub fn increment_gain(&mut self, gain: f32) {
        self.gain += gain;
    }
}

impl Effect for Overdrive {
    fn process(&self, sample: f32) -> f32 {
        self.gain * sample
    }

    fn update(&mut self, message: GenericMessage) {
        match message {
            GenericMessage::Float(field, gain) => {
                if field == "gain" {
                    self.gain = gain;
                }
            }
        }
    }
}
