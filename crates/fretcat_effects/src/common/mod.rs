use std::{fmt::Debug, cell::UnsafeCell};

use fundsp::{svf::{Svf, HighpassMode}, hacker32::highpass, prelude::An, MAX_BUFFER_SIZE};

#[derive(Clone)]
pub struct Highpass {
    filter: An<Svf<f32, f32, HighpassMode<f32>>>,
}

impl Highpass {
    fn process(&self, buffer: &mut [f32]) {
    }
}

impl Debug for Highpass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Highpass").field("freq", &self.filter.cutoff()).finish()
    }
}

impl Default for Highpass {
    fn default() -> Self {
        Self { 
            filter: highpass(),
        }
    }
}