pub mod overdrive;
mod common;

use core::fmt;
use std::ops::{Deref, DerefMut};
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect: fmt::Debug + Send + Sync {
    fn process(&self, _sample: f32) -> f32;
    fn render(&self, cx: &mut Context);
    fn height(&self) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct EffectHandle {
    handle: *mut dyn Effect
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