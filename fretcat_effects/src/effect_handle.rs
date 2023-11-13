use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;

use crate::effects::AudioEffect;

#[derive(Debug)]
pub struct EffectHandle<T: AudioEffect + ?Sized> {
    active: Arc<AtomicBool>,
    handle: Arc<T>,
}

impl<T: AudioEffect + ?Sized> Clone for EffectHandle<T> {
    fn clone(&self) -> Self {
        EffectHandle { active: self.active.clone(), handle: self.handle.clone() }
    }
}

impl<T: AudioEffect + ?Sized> Deref for EffectHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.handle.as_ref()
    }
}

impl<T: AudioEffect + ?Sized> DerefMut for EffectHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: AudioEffect + ?Sized> From<Arc<T>> for EffectHandle<T> {
    fn from(value: Arc<T>) -> Self {
        EffectHandle::new(value)
    }
}

impl<T: AudioEffect> From<Arc<dyn AudioEffect>> for EffectHandle<T> {
    fn from(value: Arc<dyn AudioEffect>) -> Self {
        EffectHandle::new(value.into_any_arc().downcast::<T>().unwrap())
    }
}

impl<T: AudioEffect> From<EffectHandle<dyn AudioEffect>> for EffectHandle<T> {
    fn from(value: EffectHandle<dyn AudioEffect>) -> Self {
        EffectHandle{ 
            handle: value.handle().into_any_arc().downcast::<T>().unwrap(),
            active: value.active.clone()
        }
    }
}

impl<T: AudioEffect + ?Sized> EffectHandle<T> {
    pub fn new(handle: Arc<T>) -> Self {
        Self {
            active: Arc::new(true.into()),
            handle: handle.clone()
        }
    }

    pub fn handle(&self) -> Arc<T> {
        self.handle.clone()
    }

    pub fn set_active(&mut self, active: bool) {
        self.active.store(active, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { Arc::as_ptr(&self.handle).cast_mut().as_mut().unwrap() }
    }
}
