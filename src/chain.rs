use std::{fmt::Debug, sync::Arc};

use nih_plug_vizia::vizia::prelude::*;

use crate::effects::{Effect, overdrive::Overdrive};

#[derive(Debug, Clone)]
pub struct Chain {
    pub chain: Vec<Arc<dyn Effect>>
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![
                Arc::new(Overdrive::default()),
                Arc::new(Overdrive::default()),
            ]
        }
    }
}

#[derive(Clone)]
pub struct ChainPtr {
    ptr: *mut Chain
}

impl Debug for ChainPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr")
            .field("chain", chain)
            .finish()
    }
}

impl ChainPtr {
    pub fn new(ptr: *mut Chain) -> Self {
        Self {
            ptr
        }
    }

    pub fn deref_mut(&self) -> &mut Chain {
        unsafe {
            &mut *self.ptr
        }
    }
}

impl Data for ChainPtr {
    fn same(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
