use core::fmt;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use fretcat_common::vizia::prelude::*;

use crate::Chain;

mod overdrive;
mod fuzz;

pub use overdrive::Overdrive;
pub use fuzz::Fuzz;

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&mut self, input_buffer: &mut [f32]);
    fn view(&self, cx: &mut Context, effect: usize);
    fn update(&self, event: &mut Event, effect: usize, chain: &mut Chain) -> Option<()>;
    fn height(&self) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);