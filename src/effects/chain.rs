use super::{Effect, overdrive::Overdrive};

#[derive(Debug)]
pub struct Chain {
    pub(crate) effects: Vec<Box<dyn Effect>>,
}

impl Default for Chain {
    fn default() -> Self {
        Self { effects: vec![
            Box::new(Overdrive::default())
        ] }
    }
}

#[derive(Debug, Clone)]
pub struct ChainPtr {
    pub(crate) ptr: *mut Chain,
}

impl ChainPtr {
    pub fn deref(&self) -> &Chain {
        unsafe {&*self.ptr}
    }

    pub fn deref_mut(&self) -> &mut Chain {
        unsafe {&mut *self.ptr}
    }
}

unsafe impl Send for ChainPtr {}
unsafe impl Sync for ChainPtr {}
