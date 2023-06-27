use core::fmt;
use std::{ops::{Deref, DerefMut}, any::Any};
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect: fmt::Debug + Send + Sync {
    fn process(&self, _sample: f32) -> f32;
    fn view(&mut self, cx: &mut Context, handle: EffectHandle);
    fn height(&self) -> f32;
    fn title(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone, Copy)]
pub struct EffectHandle {
    handle: *mut dyn Effect
}

impl EffectHandle {
    pub fn downcast_into<T: Effect + 'static>(&self) -> &T {
        let effect = unsafe {self.handle.as_ref().unwrap()};
        effect.as_any().downcast_ref::<T>().unwrap()
    }

    pub fn downcast_mut_into<T: Effect + 'static>(&mut self) -> &mut T {
        let effect = unsafe {self.handle.as_mut().unwrap()};
        effect.as_mut_any().downcast_mut::<T>().unwrap()
    }
}

impl From<&mut Box<dyn Effect>> for EffectHandle {
    fn from(value: &mut Box<dyn Effect>) -> Self {
        Self {
            handle: value.as_mut() as *mut dyn Effect
        }
    }
}

impl Deref for EffectHandle {
    type Target = dyn Effect;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.handle    
        } 
    }
}

impl DerefMut for EffectHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.handle
        }
    }
}

impl Data for EffectHandle {
    fn same(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}