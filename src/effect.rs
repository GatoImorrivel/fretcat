pub trait Effect {
    fn process(&self, _buffer: &[f32], _sample: f32) -> f32;
}

pub struct Overdrive {
    gain: f32,
}

impl Effect for Overdrive {
    fn process(&self, _buffer: &[f32], _sample: f32) -> f32 {
        return self.gain * _sample;
    }
}