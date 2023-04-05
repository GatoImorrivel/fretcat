

pub trait Effect {
    fn process(&self, sample: f32) -> f32;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Overdrive {
    gain: f32
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
}