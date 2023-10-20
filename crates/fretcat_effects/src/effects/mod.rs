use core::fmt;
use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::DynClone;
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

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + Downcast {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]));
    #[allow(unused_variables)]
    fn view(&self, cx: &mut Context, effect: usize) {}
    #[allow(unused_variables)]
    fn update(&mut self, event: &mut Event) -> Option<()> {
        Some(())
    }
    fn height(&self) -> f32 {
        0.0
    }
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);
