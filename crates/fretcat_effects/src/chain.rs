use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::{effect::{Effect, EffectHandle}, overdrive::Overdrive, fuzz::Fuzz};

#[derive(Debug)]
pub struct Chain {
    pub chain: Vec<Box<dyn Effect>>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![
                Box::new(Overdrive::default()),
                Box::new(Fuzz::default()),
                Box::new(Overdrive::default()),
                Box::new(Overdrive::default()),
                Box::new(Fuzz::default()),
            ],
        }
    }
}

#[derive(Clone, Lens)]
pub struct ChainHandle {
    pub ptr: *mut Chain,
    pub effects: Vec<EffectHandle>
}

impl ChainHandle {
    pub fn new(ptr: *mut Chain) -> Self {
        let chain = unsafe {&mut *ptr};
        let effects = chain.chain.iter_mut().map(|effect| {
            EffectHandle::from(effect)
        }).collect();

        Self {
            ptr,
            effects
        }
    }
}

impl Model for ChainHandle {}

impl Debug for ChainHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr")
            .field("effects", &self.effects)
            .field("chain", chain)
            .finish()
    }
}

impl Deref for ChainHandle {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for ChainHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

unsafe impl Send for ChainHandle {}
unsafe impl Sync for ChainHandle {}
