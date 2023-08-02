use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    effect::{AudioEffect, Effect},
    overdrive::Overdrive,
};

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct Chain {
    pub effects: Vec<Effect>,
    cache: HashMap<Effect, Box<dyn AudioEffect>>,
}

impl Chain {
    pub fn insert(&mut self, audio_effect: Box<dyn AudioEffect>) -> (usize, Effect) {
        let e = Effect::new();
        self.effects.push(e);
        self.cache.insert(e, audio_effect);
        self.effects.clone().into_iter().enumerate().last().unwrap()
    }

    pub fn insert_at(&mut self, index: usize, audio_effect: Box<dyn AudioEffect>) -> Effect {
        let e = Effect::new();
        self.effects.insert(index, e);
        self.cache.insert(e, audio_effect);
        self.effects.clone()[index]
    }

    pub fn remove(&mut self, effect: &Effect) {
        let fetch = self.effects.clone().into_iter().enumerate().find(|(_i, e)| {
            e == effect
        });

        if let Some((index, effect)) = fetch {
            self.effects.remove(index);
            self.cache.remove(&effect);
        }
    }

    pub fn query(&self, effect: &Effect) -> Option<&Box<dyn AudioEffect>> {
        let c = match self.cache.get(&effect) {
            Some(c) => c,
            None => return None,
        };

        Some(c)
    }

    pub fn query_mut(&mut self, effect: &Effect) -> Option<&mut Box<dyn AudioEffect>> {
        let c = match self.cache.get_mut(&effect) {
            Some(c) => c,
            None => return None,
        };

        Some(c)
    }

    pub fn query_cast<T: AudioEffect + 'static>(&self, effect: &Effect) -> Option<&T> {
        self.query(effect)?.as_any().downcast_ref::<T>()
    }

    pub fn query_cast_mut<T: AudioEffect + 'static>(&mut self, effect: &Effect) -> Option<&mut T> {
        self.query_mut(effect)?.as_mut_any().downcast_mut::<T>()
    }

    pub fn query_index(&self, index: usize) -> Option<(Effect, &Box<dyn AudioEffect>)> {
        let e = match self.effects.get(index) {
            Some(e) => e,
            None => return None,
        };

        let data = self.cache.get(e).unwrap();
        Some((*e, data))
    }

    pub fn handle(&mut self) -> ChainHandle {
        ChainHandle {
            ptr: self as *mut _,
            redraw: 0
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        let mut chain = Self {
            effects: vec![],
            cache: HashMap::new(),
        };

        chain.insert(Box::new(Overdrive::default()));

        chain
    }
}

pub enum ChainEvent {
    Insert(Box<dyn AudioEffect>, usize),
    Remove(Effect),
}

#[derive(Clone, Lens)]
pub struct ChainHandle {
    ptr: *mut Chain,
    pub redraw: u32
}

impl Model for ChainHandle {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        let e = event.take();
        if let Some(e) = e {
            match e {
                ChainEvent::Insert(data, pos) => {
                    self.insert_at(pos, data);

                    nih_log!("{:#?}", self);
                }
                ChainEvent::Remove(effect) => {
                    self.remove(&effect);
                    nih_log!("{:#?}", self);
                }
            }

            self.redraw += 1;
        }
    }
}

impl Debug for ChainHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = unsafe { &*self.ptr };
        f.debug_struct("ChainPtr")
            .field("ptr", &self.ptr)
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
