use std::f32::consts::PI;

use serde::{Serialize, Deserialize};

// this is from the fundsp crate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SvfCoeffs {
    pub a1: f32,
    pub a2: f32,
    pub a3: f32,
    pub m0: f32,
    pub m1: f32,
    pub m2: f32,
}

impl SvfCoeffs {
    /// Calculate coefficients for a lowpass filter.
    pub fn lowpass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 0.0;
        let m1 = 0.0;
        let m2 = 1.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a highpass filter.
    pub fn highpass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = -k;
        let m2 = -1.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a bandpass filter.
    pub fn bandpass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 0.0;
        let m1 = 1.0;
        let m2 = 0.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a notch filter.
    pub fn notch(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = -k;
        let m2 = 0.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a peak filter.
    pub fn peak(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = -k;
        let m2 = -2.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for an allpass filter.
    pub fn allpass(sample_rate: f32, cutoff: f32, q: f32) -> Self {
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = -2.0 * k;
        let m2 = 0.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a bell filter.
    /// Gain is amplitude gain (gain > 0).
    pub fn bell(sample_rate: f32, cutoff: f32, q: f32, gain: f32) -> Self {
        let a = gain.sqrt();
        let g = (PI * cutoff / sample_rate).tan();
        let k = 1.0 / (q * a);
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = k * (a * a - 1.0);
        let m2 = 0.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a low shelf filter.
    /// Gain is amplitude gain (gain > 0).
    pub fn lowshelf(sample_rate: f32, cutoff: f32, q: f32, gain: f32) -> Self {
        let a = gain.sqrt();
        let g = (PI * cutoff / sample_rate).tan() / a.sqrt();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = 1.0;
        let m1 = k * (a - 1.0);
        let m2 = a * a - 1.0;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }

    /// Calculate coefficients for a high shelf filter.
    /// Gain is amplitude gain (gain > 0).
    pub fn highshelf(sample_rate: f32, cutoff: f32, q: f32, gain: f32) -> Self {
        let a = gain.sqrt();
        let g = (PI * cutoff / sample_rate).sqrt() * a.sqrt();
        let k = 1.0 / q;
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = a * a;
        let m1 = k * (1.0 - a) * a;
        let m2 = 1.0 - a * a;

        SvfCoeffs {
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
}