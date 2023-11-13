use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::interpolate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Delay {
    feedback: f32,
    delay_time_secs: f32,
    current_write_position: usize,
    current_read_position: usize,
    sample_rate: f32,
    buffer_size: usize,
    delay_buffer: Vec<f32>,
    max_delay_time: Duration,
}

impl Default for Delay {
    fn default() -> Self {
        let sample_rate = 44100.0;
        let max_delay_secs = 1;
        let delay_time_secs = 0.2;
        Self {
            feedback: 0.3,
            delay_time_secs,
            current_write_position: (delay_time_secs * sample_rate) as usize,
            sample_rate,
            buffer_size: max_delay_secs * sample_rate as usize,
            current_read_position: 0,
            delay_buffer: Self::make_vec(max_delay_secs * sample_rate as usize),
            max_delay_time: Duration::from_secs(1),
        }
    }
}

impl Delay {
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.buffer_size = self.max_delay_time.as_secs() as usize * self.sample_rate as usize;
        self.delay_buffer = Self::make_vec(self.buffer_size);
        self.current_write_position = self.delay_time_secs as usize * self.sample_rate as usize;
    }

    pub fn feedback(&self) -> f32 {
        self.feedback
    }

    pub fn delay_time_secs(&self) -> f32 {
        self.delay_time_secs
    }

    fn make_vec(max_delay_time: usize) -> Vec<f32> {
        let mut v = Vec::with_capacity(max_delay_time);
        v.resize(max_delay_time, 0.0.into());
        v
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn set_delay_time_secs(&mut self, value: f32) {
        self.delay_time_secs = value;

        let write_position = self.current_write_position as i64;
        let sample_rate = self.sample_rate;
        let buffer_size = self.buffer_size as i64;
        let offset = (sample_rate * value) as i64;
        let cursor = write_position - offset + buffer_size;
        self.current_read_position = (cursor % buffer_size) as usize;
    }

    pub fn read(&mut self) -> f32 {
        let delay_samples = self.delay_samples();
        let offset = delay_samples - delay_samples.floor();
        let buffer_size = self.buffer_size;

        let mut current_read_position = self.current_read_position;
        let delay_output = interpolate(
            self.delay_buffer[current_read_position],
            self.delay_buffer[(current_read_position + 1) % buffer_size],
            offset,
        );

        current_read_position += 1;
        if current_read_position >= buffer_size {
            current_read_position = 0;
        }
        self.current_read_position = current_read_position;

        delay_output
    }

    pub fn write(&mut self, sample: f32) {
        let mut current_write_position = self.current_write_position;
        self.delay_buffer[current_write_position] = sample;

        current_write_position += 1;
        if current_write_position >= self.buffer_size {
            current_write_position = 0;
        }
        self.current_write_position = current_write_position;
    }

    fn delay_samples(&self) -> f32 {
        self.delay_time_secs * self.sample_rate
    }

    #[inline]
    pub fn tick(&mut self, sample: f32) -> f32 {
        let out = self.read();

        let write = sample + out * self.feedback;
        self.write(write);

        out
    }
}
