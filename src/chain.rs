use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::effects::{overdrive::Overdrive, Effect};

#[derive(Debug, Clone, Lens)]
pub struct Chain {
    pub chain: Vec<Arc<dyn Effect>>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
            ],
        }
    }
}

#[derive(Clone)]
pub struct ChainPtr {
    ptr: *mut Chain,
}

impl ChainPtr {
    pub fn new(ptr: *mut Chain) -> Self {
        Self { ptr }
    }

    pub fn render<L: Lens<Target = Self>>(cx: &mut Context, lens: L) {
        for effect in &mut lens.get(cx).deref_mut().chain {
            effect.render(cx);
        }
    }
}

impl Debug for ChainPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr").field("chain", chain).finish()
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

impl Data for ChainPtr {
    fn same(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
