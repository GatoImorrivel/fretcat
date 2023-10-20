use fretcat_macros::Message;

use nih_plug::{vizia::prelude::*, util::db_to_gain_fast, nih_log};

use super::AudioEffect;

#[derive(Debug, Clone, Copy, Message)]
pub struct Gain {
    #[msg]
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