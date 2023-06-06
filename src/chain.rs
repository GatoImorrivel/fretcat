use crate::effect::Effect;

pub struct Chain {
    chain: Vec<Box<dyn Effect + Send + Sync>>
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![]
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
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
