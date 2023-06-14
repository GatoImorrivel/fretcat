use std::fmt::Debug;

use nih_plug_vizia::vizia::prelude::*;

use crate::effect::{Effect, overdrive::Overdrive};

#[derive(Debug)]
pub struct Chain {
    pub chain: Vec<Box<dyn Effect + Send + Sync>>
}

impl Default for Chain {
    fn default() -> Self {
        let test = vec![5];
        Self {
            chain: vec![
                Box::new(Overdrive::default())
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

    pub fn deref(&self) -> &Chain {
        unsafe {
            &*self.ptr
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
