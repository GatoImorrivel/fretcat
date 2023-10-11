use core::fmt;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use nih_plug::vizia::prelude::*;

// Distortions
mod fuzz;
mod overdrive;

// Reverbs
mod studioreverb;

#[cfg(feature = "simulate")]
mod input_simulator;
#[cfg(feature = "simulate")]
pub use input_simulator::InputSimulator;

pub use overdrive::Overdrive;
pub use fuzz::Fuzz;
pub use studioreverb::StudioReverb;

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]));
    fn view(&self, cx: &mut Context, effect: usize);
    fn update(&mut self, event: &mut Event) -> Option<()>;
    fn height(&self) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);