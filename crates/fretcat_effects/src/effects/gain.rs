

use nih_plug::util::db_to_gain_fast;

use super::AudioEffect;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gain {
    pub gain_in_db: f32
}

impl Default for Gain {
    fn default() -> Self {
        Self {
            gain_in_db: 1.0
        }
    }
}

impl AudioEffect for Gain {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32])) {
        input_buffer.0.iter_mut().zip(input_buffer.1.iter_mut()).for_each(|(left, right)| {
            *left *= db_to_gain_fast(self.gain_in_db);
            *right *= db_to_gain_fast(self.gain_in_db);
        });
    }
}