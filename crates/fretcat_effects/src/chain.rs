use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::{
    effect::{Effect, EffectHandle},
    fuzz::Fuzz,
    overdrive::Overdrive,
};

#[derive(Debug)]
pub struct Chain {
    pub chain: Vec<Box<dyn Effect>>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            chain: vec![Box::new(Overdrive::default())],
        }
    }
}

#[derive(Clone, Lens)]
pub struct ChainHandle {
    pub ptr: *mut Chain,
    pub effects: Vec<EffectHandle>,
}

impl ChainHandle {
    pub fn new(ptr: *mut Chain) -> Self {
        let chain = unsafe { &mut *ptr };
        let effects = chain
            .chain
            .iter_mut()
            .map(|effect| EffectHandle::from(effect))
            .collect();

        Self { ptr, effects }
    }

    pub fn push_effect(&mut self, mut effect: Box<dyn Effect>) {
        let handle = EffectHandle::from(&mut effect);
        self.effects.push(handle);
        self.chain.push(effect);
    }

    pub fn insert_effect(&mut self, index: usize, mut effect: Box<dyn Effect>) {
        let handle = EffectHandle::from(&mut effect);
        self.effects.insert(index, handle);
        self.chain.insert(index, effect);
    }

    pub fn remove_effect(&mut self, index: usize) {
        self.chain.remove(index);
        self.effects.remove(index);
    }
}

pub enum ChainEvent {
    PushEffect(Box<dyn Effect>),
    InsertEffect(usize, Box<dyn Effect>),
    RemoveEffect(usize)
}

impl Model for ChainHandle {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let e = event.take();
        if let Some(e) = e {
            match e {
                ChainEvent::PushEffect(effect) => {
                    self.push_effect(effect);
                },
                ChainEvent::InsertEffect(index, effect) => {
                    self.insert_effect(index, effect);
                },
                ChainEvent::RemoveEffect(index) => {
                    self.remove_effect(index);
                }
            }
        }
    }
}

impl Debug for ChainHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr")
            .field("effects", &self.effects)
            .field("chain", chain)
            .finish()
    }
}

impl Deref for ChainHandle {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for ChainHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

unsafe impl Send for ChainHandle {}
unsafe impl Sync for ChainHandle {}
