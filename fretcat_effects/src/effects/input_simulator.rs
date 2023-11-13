use crate::frame::Frame;

use nih_plug::{prelude::*, util::db_to_gain};

use super::AudioEffect;

#[derive(Debug, Clone)]
pub struct InputSimulator {
    samples: Vec<f32>,
    current_sample: usize,
}

impl Default for InputSimulator {
    fn default() -> Self {
        let mut wav = hound::WavReader::open("eletricguitar.wav").unwrap();
        nih_plug::nih_log!("{:#?}", wav.spec());
        let input = wav.samples::<i16>().map(|s| s.unwrap() as f32 / db_to_gain(110.0)).collect::<Vec<_>>();
        Self {
            samples: input,
            current_sample: 0
        }
    }
}

impl InputSimulator {
    pub fn new(input: Vec<f32>) -> Self {
        Self {
            samples: input,
            current_sample: 0usize,
        }
    }

    pub fn tick(&mut self) -> f32 {
        if self.current_sample >= self.samples.len() {
            self.current_sample = 0;
        }
        let current = self.samples[self.current_sample];
        self.current_sample += 1;
        current 
    }
}

impl AudioEffect for InputSimulator {
    fn process(&mut self, input_buffer: &mut Frame, _transport: &Transport) {
        input_buffer.process_individual(|left, right| {
            *left += self.tick();
            *right += self.tick();
        });
    }
}