use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::effects::{overdrive::Overdrive, Effect};

#[derive(Debug)]
pub struct Chain {
    pub chain: Vec<Box<dyn Effect>>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![
                Box::new(Overdrive::default()),
                Box::new(Overdrive::default()),
                Box::new(Overdrive::default()),
            ],
        }
    }
}

#[derive(Clone, Lens)]
pub struct ChainPtr {
    ptr: *mut Chain,
    pub(crate) effects_ptr: Vec<*mut dyn Effect>
}

impl ChainPtr {
    pub fn new(ptr: *mut Chain) -> Self {
        let chain = unsafe {&mut *ptr};
        let effects_ptr = chain.chain.iter_mut().map(|effect| {
            effect.as_mut() as *mut dyn Effect
        }).collect();

        Self {
            ptr,
            effects_ptr 
        }
    }
}

impl Model for ChainPtr {}

impl Debug for ChainPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr")
            .field("effects_ptr", &self.effects_ptr)
            .field("chain", chain)
            .finish()
    }
}

impl Deref for ChainPtr {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for ChainPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
