use core::fmt;
use std::any::Any;
use downcast_rs::{Downcast, DowncastSync, impl_downcast};
use dyn_clone::DynClone;
use rand::random;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Effect(u64);

impl Effect {
    pub fn new() -> Self {
        Self(random())
    }
}

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&self, _sample: f32) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);