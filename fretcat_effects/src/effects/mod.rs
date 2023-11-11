use core::fmt;
use std::sync::Arc;
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
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport);
    fn view(&self, _cx: &mut Context, _effect: Arc<dyn AudioEffect>) {}
    fn height(&self) -> f32 {
        0.0
    }
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);
