use rustfft::{num_complex::Complex, FftPlanner};
use serde::{Deserialize, Serialize};
use std::{f64::consts::PI, fmt::Debug};
use textplots::Plot;

use crate::components::Point;

use super::coeffs::SvfCoeffs;

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

    pub fn frequency_response(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let num_points = 1000;
        let nyquist = (self.sample_rate() / 2.0) as f64;

        for i in 0..nyquist as i32 {
            let normalized_freq = i as f64 / nyquist;
            let y = {
                if (normalized_freq * nyquist) as i32 > self.cutoff() as i32 {
                    0.0
                } else {
                    1.0
                }
            };
            points.push(Point::new((normalized_freq * nyquist).round() as i32, y as i32));
        }

        points
    }
}
