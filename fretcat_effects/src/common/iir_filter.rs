use std::f32::consts::{PI, SQRT_2, TAU};

use rustfft::num_complex::Complex32;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BiquadCoefs {
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
}

impl BiquadCoefs {
    /// Returns settings for a Butterworth lowpass filter.
    /// Cutoff is the -3 dB point of the filter in Hz.
    pub fn butter_lowpass(sample_rate: f32, cutoff: f32) -> Self {
        let f = (cutoff * PI / sample_rate).tan();
        let a0r = 1.0 / (1.0 + SQRT_2 * f + f * f);
        let a1 = (2.0 * f * f - 2.0) * a0r;
        let a2 = (1.0 - SQRT_2 * f + f * f) * a0r;
        let b0 = f * f * a0r;
        let b1 = 2.0 * b0;
        let b2 = b0;
        Self { a1, a2, b0, b1, b2 }
    }

    /// Returns settings for a constant-gain bandpass resonator.
    /// The center frequency is given in Hz.
    /// Bandwidth is the difference in Hz between -3 dB points of the filter response.
    /// The overall gain of the filter is independent of bandwidth.
    pub fn resonator(sample_rate: f32, center: f32, bandwidth: f32) -> Self {
        let r = (-PI * bandwidth / sample_rate).exp();
        let a1 = -2.0 * r * (TAU * center / sample_rate).cos();
        let a2 = r * r;
        let b0 = (1.0 - r * r).sqrt() * 0.5;
        let b1 = 0.0;
        let b2 = -b0;
        Self { a1, a2, b0, b1, b2 }
    }

    /// Arbitrary biquad.
    pub fn arbitrary(a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) -> Self {
        Self { a1, a2, b0, b1, b2 }
    }

    /// Frequency response at frequency `omega` expressed as fraction of sampling rate.
    pub fn response(&self, omega: f32) -> Complex32 {
        let z1 = Complex32::from_polar(1.0, -TAU * omega);
        let z2 = z1 * z1;
        /// Complex64 with real component `x` and imaginary component zero.
        fn re(x: f32) -> Complex32 {
            Complex32::new(x, 0.0)
        }
        (re(self.b0) + re(self.b1) * z1 + re(self.b2) * z2)
            / (re(1.0) + re(self.a1) * z1 + re(self.a2) * z2)
    }
}

/// 2nd order IIR filter implemented in normalized Direct Form I.
/// Setting: coefficients as tuple (a1, a2, b0, b1, b2).
/// - Input 0: input signal.
/// - Output 0: filtered signal.
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct Biquad {
    coefs: BiquadCoefs,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    sample_rate: f32,
}

impl Biquad {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            ..Default::default()
        }
    }

    pub fn with_coefs(coefs: BiquadCoefs) -> Self {
        Self {
            coefs,
            sample_rate: 44100.0,
            ..Default::default()
        }
    }

    pub fn coefs(&self) -> &BiquadCoefs {
        &self.coefs
    }

    pub fn set_coefs(&mut self, coefs: BiquadCoefs) {
        self.coefs = coefs;
    }

    pub fn set(&mut self, a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) {
        self.set_coefs(BiquadCoefs::arbitrary(a1, a2, b0, b1, b2));
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        let x0 = input;
        let y0 = self.coefs.b0 * x0 + self.coefs.b1 * self.x1 + self.coefs.b2 * self.x2
            - self.coefs.a1 * self.y1
            - self.coefs.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x0;
        self.y2 = self.y1;
        self.y1 = y0;
        y0
    }
}

/// Butterworth lowpass filter.
/// Setting: cutoff.
/// Number of inputs is `N`, either `U1` or `U2`.
/// - Input 0: input signal
/// - Input 1 (optional): cutoff frequency (Hz)
/// - Output 0: filtered signal
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct ButterLowpass {
    biquad: Biquad,
    sample_rate: f32,
    cutoff: f32,
}

impl ButterLowpass {
    /// Create new Butterworth lowpass filter with initial `cutoff` frequency in Hz.
    pub fn new(cutoff: f32) -> Self {
        let mut node = ButterLowpass {
            biquad: Biquad::new(),
            sample_rate: 44100.0,
            cutoff: 0.0,
        };
        node.biquad.reset();
        node.set_cutoff(cutoff);
        node
    }

    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.biquad
            .set_coefs(BiquadCoefs::butter_lowpass(self.sample_rate, cutoff));
        self.cutoff = cutoff;
    }

    pub fn set(&mut self, setting: f32) {
        self.set_cutoff(setting);
    }

    pub fn reset(&mut self) {
        self.biquad.reset();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.biquad.set_sample_rate(sample_rate);
        self.set_cutoff(self.cutoff);
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn cutoff(&self) -> f32 {
        self.cutoff
    }

    #[inline]
    pub fn tick(&mut self, input: f32) -> f32 {
        self.biquad.tick(input)
    }
}

/// Constant-gain bandpass filter (resonator).
/// Filter gain is (nearly) independent of bandwidth.
/// Setting: (center, bandwidth).
/// Number of inputs is `N`, either `U1` or `U3`.
/// - Input 0: input signal
/// - Input 1 (optional): filter center frequency (peak) (Hz)
/// - Input 2 (optional): filter bandwidth (distance) between -3 dB points (Hz)
/// - Output 0: filtered signal
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Resonator {
    biquad: Biquad,
    sample_rate: f32,
    center: f32,
    bandwidth: f32,
}

impl Resonator {
    /// Create new resonator bandpass. Initial `center` frequency and `bandwidth` are specified in Hz.
    pub fn new(center: f32, bandwidth: f32) -> Self {
        let mut node = Resonator {
            biquad: Biquad::new(),
            sample_rate: 44100.0,
            center,
            bandwidth,
        };
        node.biquad.reset();
        node.set_center_bandwidth(center, bandwidth);
        node
    }

    pub fn set_center_bandwidth(&mut self, center: f32, bandwidth: f32) {
        self.biquad
            .set_coefs(BiquadCoefs::resonator(self.sample_rate, center, bandwidth));
        self.center = center;
        self.bandwidth = bandwidth;
    }

    pub fn center_bandwidth(&self) -> f32 {
        self.center
    }

    pub fn set(&mut self, center: f32, bandwidth: f32) {
        self.set_center_bandwidth(center, bandwidth);
    }

    pub fn reset(&mut self) {
        self.biquad.reset();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.set_center_bandwidth(self.center, self.bandwidth);
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    #[inline]
    pub fn tick(
        &mut self,
        input: f32,
    ) -> f32 {
        self.biquad.tick(input)
    }
}

/// DC blocking filter with cutoff frequency in Hz.
/// Setting: cutoff.
/// - Input 0: signal
/// - Output 0: zero centered signal
#[derive(Debug, Default, Clone, Serialize, Deserialize, Copy)]
pub struct DCBlock {
    x1: f32,
    y1: f32,
    cutoff: f32,
    coeff: f32,
    sample_rate: f32,
}

impl DCBlock {
    /// Create new DC blocking filter with `cutoff` frequency specified in Hz.
    pub fn new(cutoff: f32) -> Self {
        let mut node = DCBlock {
            cutoff,
            ..Default::default()
        };
        node.reset();
        node.set_sample_rate(44100.0);
        node
    }

    /// Set the cutoff frequency (in Hz).
    pub fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
        self.coeff = 1.0 - TAU / self.sample_rate * cutoff;
    }

    pub fn cutoff(&self) -> f32 {
        self.cutoff
    }

    pub fn set(&mut self, setting: f32) {
        self.set_cutoff(setting);
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.set_cutoff(self.cutoff);
    }

    pub fn sample_rate(&mut self) -> f32 {
        self.sample_rate
    }

    #[inline]
    pub fn tick(
        &mut self,
        input: f32,
    ) -> f32 {
        let x = input;
        let y0 = x - self.x1 + self.coeff * self.y1;
        self.x1 = x;
        self.y1 = y0;
        y0
    }
}
