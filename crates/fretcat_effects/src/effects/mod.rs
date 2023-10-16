use core::fmt;
use downcast_rs::{impl_downcast, Downcast};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PreFX(pub &'static str);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PostFX(pub &'static str);

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + Downcast {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]));
    fn view(&self, cx: &mut Context, effect: usize);
    fn update(&mut self, event: &mut Event) -> Option<()>;
    fn height(&self) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);