use nih_plug_vizia::vizia::state::Data;

use crate::effect::{Effect, Overdrive};

pub struct Chain {
    pub chain: Vec<Box<dyn Effect + Send + Sync>>
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![
                Box::new(Overdrive::default()),
                Box::new(Overdrive::default())
            ]
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChainPtr {
    ptr: *mut Chain
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
