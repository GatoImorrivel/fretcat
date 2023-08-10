use atomic_refcell::AtomicRefCell;
use core::fmt;
use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use dyn_clone::DynClone;
use nih_plug_vizia::vizia::prelude::*;
use rand::random;
use std::{any::Any, sync::Arc};

use crate::{Chain, chain::ChainHandle};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Effect(u64);

impl Effect {
    pub fn new() -> Self {
        Self(random())
    }
}

pub trait AudioEffect: fmt::Debug + Send + Sync + DynClone + DowncastSync {
    fn process(&self, _sample: f32) -> f32;
    fn view(&self, cx: &mut Context, effect: Effect);
    fn update(&self, event: &mut Event, effect: Effect, chain: &mut Chain) -> Option<()>;
    fn height(&self) -> f32;
}

impl_downcast!(AudioEffect);
dyn_clone::clone_trait_object!(AudioEffect);
