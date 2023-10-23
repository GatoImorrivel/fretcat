use core::fmt;
use std::sync::Arc;
use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use dyn_clone::DynClone;
use fretcat_macros::*;
use nih_plug::vizia::prelude::*;

// Distortions
mod fuzz;
mod gain;
mod overdrive;

// Reverbs
mod studioreverb;

#[cfg(feature = "simulate")]
mod input_simulator;
#[cfg(feature = "simulate")]
pub use input_simulator::InputSimulator;

pub use fuzz::Fuzz;
pub use gain::Gain;
pub use overdrive::Overdrive;
pub use studioreverb::StudioReverb;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PreFX(pub &'static str);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PostFX(pub &'static str);

use core::any::Any;

pub trait DynPartialEq {
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]));
    fn view(&self, _cx: &mut Context, _effect: Arc<dyn AudioEffect>) {}
    fn height(&self) -> f32 {
        0.0
    }
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);
