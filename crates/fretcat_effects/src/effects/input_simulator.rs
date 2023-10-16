use nih_plug::{vizia::prelude::*, nih_log};

use super::AudioEffect;

#[derive(Debug, Clone)]
pub struct InputSimulator {
    samples: Vec<f32>,
    current_sample: usize,
}

impl Default for InputSimulator {
    fn default() -> Self {
        let mut wav = hound::WavReader::open("H.wav").unwrap();
        let input = wav.samples::<i16>().map(|s| s.unwrap() as f32).collect::<Vec<_>>();
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

    pub fn tick(&mut self) -> (f32, f32) {
        if self.current_sample >= self.samples.len() {
            self.current_sample = 0;
        }
        let left = self.samples[self.current_sample];
        self.current_sample += 1;
        let right = self.samples[self.current_sample];
        self.current_sample += 1;
        (left, right)
    }
}

impl AudioEffect for InputSimulator {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32])) {
        input_buffer.0.iter_mut().zip(input_buffer.1.iter_mut()).for_each(|(left, right)| {
            (*left, *right) = self.tick();
        });
    }

    fn update(&mut self, event: &mut Event) -> Option<()> {
        Some(())
    }

    fn view(&self, cx: &mut Context, effect: usize) {
        
    }

    fn height(&self) -> f32 {
        0.0
    }
}