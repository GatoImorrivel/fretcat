mod coeffs;

use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use coeffs::SvfCoeffs;

 pub fn map_normalized_value(value: f32, min_output: f32, max_output: f32) -> f32 {
    min_output + value * (max_output - min_output)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilterMode {
    Highpass,
    Lowpass
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    coeffs: SvfCoeffs,
    ic1eq: f32,
    ic2eq: f32,
    sample_rate: f32,
    cutoff: f32,
    q: f32,
    mode: FilterMode
}

impl Filter {
    pub fn new(mode: FilterMode, sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let coeffs = Filter::get_coeffs(mode, sample_rate, cutoff, q);

        Self { 
            coeffs,
            ic1eq: 0.0,
            ic2eq: 0.0,
            cutoff,
            q,
            sample_rate,
            mode
        }
    }

    #[inline]
    pub fn process(&self, input: &mut [f32]) {
        for i in 0..input.len() {
            input[i] = self.tick(input[i]);
        }
    }

    #[inline]
    pub fn tick(&self, sample: f32) -> f32 {
        let v0 = sample;
        let v3 = v0 - self.ic2eq;
        let v1 = self.coeffs.a1 * self.ic1eq + self.coeffs.a2 * v3;
        let v2 = self.ic2eq + self.coeffs.a2 * self.ic1eq + self.coeffs.a3 * v3;
        let ic1eq_ptr = &self.ic1eq as *const f32 as *mut f32;
        let ic2eq_ptr = &self.ic2eq as *const f32 as *mut f32;
        unsafe {
            *ic1eq_ptr = 2.0 * v1 - self.ic1eq;
            *ic2eq_ptr = 2.0 * v2 - self.ic2eq;
        }
        self.coeffs.m0 * v0 + self.coeffs.m1 * v1 + self.coeffs.m2 * v2
    }

    pub fn recalculate_coeffs(&mut self, cutoff: f32, q: f32) {
        self.cutoff = cutoff;
        self.q = q;
        self.coeffs = Filter::get_coeffs(self.mode, self.sample_rate, cutoff, q);
    }

    fn get_coeffs(mode: FilterMode, sample_rate: f32, cutoff: f32, q: f32) -> SvfCoeffs {
        match mode {
            FilterMode::Highpass => {
                SvfCoeffs::highpass(sample_rate, cutoff, q)
            }
            FilterMode::Lowpass => {
                SvfCoeffs::lowpass(sample_rate, cutoff, q)
            }
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
}
