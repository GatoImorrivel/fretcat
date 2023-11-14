use nih_plug::util::gain_to_db;
use rustfft::{num_complex::Complex, FftPlanner};
use serde::{de, Deserialize, Serialize};
use std::{f32::consts::PI, fmt::Debug};
use textplots::Plot;

use crate::components::Point;

use super::{coeffs::SvfCoeffs, normalize_value};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterMode {
    Highpass,
    Lowpass,
    BandPass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AudioFilter {
    coeffs: SvfCoeffs,
    ic1eq: f32,
    ic2eq: f32,
    sample_rate: f32,
    cutoff: f32,
    q: f32,
    mode: FilterMode,
}

impl AudioFilter {
    pub fn new(mode: FilterMode, sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let coeffs = AudioFilter::get_coeffs(mode, sample_rate, cutoff, q);

        Self {
            coeffs,
            ic1eq: 0.0,
            ic2eq: 0.0,
            cutoff,
            q,
            sample_rate,
            mode,
        }
    }

    #[inline]
    pub fn mode(&self) -> FilterMode {
        self.mode
    }

    #[inline]
    pub fn process(&mut self, input: &mut [f32]) {
        for i in 0..input.len() {
            input[i] = self.tick(input[i]);
        }
    }

    #[inline]
    pub fn tick(&mut self, sample: f32) -> f32 {
        let v0 = sample;
        let v3 = v0 - self.ic2eq;
        let v1 = self.coeffs.a1 * self.ic1eq + self.coeffs.a2 * v3;
        let v2 = self.ic2eq + self.coeffs.a2 * self.ic1eq + self.coeffs.a3 * v3;
        self.ic1eq = 2.0 * v1 - self.ic1eq;
        self.ic2eq = 2.0 * v2 - self.ic2eq;
        self.coeffs.m0 * v0 + self.coeffs.m1 * v1 + self.coeffs.m2 * v2
    }

    pub fn recalculate_coeffs(&mut self, cutoff: f32, q: f32, sample_rate: f32) {
        self.cutoff = cutoff;
        self.q = q;
        self.coeffs = AudioFilter::get_coeffs(self.mode, sample_rate, cutoff, q);
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.recalculate_coeffs(cutoff, self.q(), self.sample_rate());
    }

    pub fn set_q(&mut self, q: f32) {
        self.recalculate_coeffs(self.cutoff(), q, self.sample_rate());
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.recalculate_coeffs(self.cutoff(), self.q(), sample_rate);
    }

    fn get_coeffs(mode: FilterMode, sample_rate: f32, cutoff: f32, q: f32) -> SvfCoeffs {
        match mode {
            FilterMode::Highpass => SvfCoeffs::highpass(sample_rate, cutoff, q),
            FilterMode::Lowpass => SvfCoeffs::lowpass(sample_rate, cutoff, q),
            FilterMode::BandPass => SvfCoeffs::bandpass(sample_rate, cutoff, q),
        }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn cutoff(&self) -> f32 {
        self.cutoff
    }

    pub fn q(&self) -> f32 {
        self.q
    }

    pub fn graph(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let duration = 0.01; // Duration in seconds

        let num_samples = (self.sample_rate() * duration) as usize;

        let mut sine_wave: Vec<f32> = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let time = i as f32 / self.sample_rate();
            let min_frequency = 20.0;
            let max_frequency = self.sample_rate() / 2.0; // Nyquist frequency
            let frequency = min_frequency + (max_frequency - min_frequency) * time / duration; // Linearly increasing frequency
            let amplitude = 100.0;

            let sample = amplitude * (2.0 * PI * frequency * time).sin();

            sine_wave.push(sample);
        }

        let mut cloned = Self::new(self.mode(), self.sample_rate(), self.cutoff(), self.q());

        sine_wave
            .iter_mut()
            .for_each(|sample| *sample = cloned.tick(*sample));

        let sine_len = sine_wave.len();
        let half = sine_wave.into_iter().take(sine_len / 2);

        (0..num_samples / 2)
            .into_iter()
            .zip(half)
            .for_each(|(x, y)| {
                points.push(Point::new(x as i32, (y * 1000.0).round() as i32));
            });

        points
    }
}
