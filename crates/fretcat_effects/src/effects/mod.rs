use core::fmt;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use nih_plug_vizia::vizia::prelude::*;
use rand::random;

use crate::Chain;

mod overdrive;
mod fuzz;

pub use overdrive::Overdrive;
pub use fuzz::Fuzz;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Effect(u64);

impl Effect {
    pub fn new() -> Self {
        Self(random())
    }
}

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&self, input_buffer: &mut [f32]);
    fn view(&self, cx: &mut Context, effect: Effect);
    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()>;
    fn height(&self) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);