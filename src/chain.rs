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

    pub fn get_chain(&self) -> &Chain {
        unsafe {
            &*self.ptr
        }
    }
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
