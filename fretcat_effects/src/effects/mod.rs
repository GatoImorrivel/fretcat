use core::fmt;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;

use nih_plug::vizia::prelude::*;


mod distortion;
pub use distortion::*;
mod delay;
pub use delay::*;
mod dynamics;
pub use dynamics::*;
mod reverb;
pub use reverb::*;

mod input_simulator;
pub use input_simulator::InputSimulator;


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PreFX(pub &'static str);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Data)]
pub struct PostFX(pub &'static str);

use crate::{frame::Frame, effect_handle::EffectHandle};

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport);
    #[allow(unused_variables)]
    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {}
    fn height(&self) -> f32 {
        0.0
    }
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);
