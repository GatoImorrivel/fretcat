use std::sync::Arc;

use nih_plug::vizia::prelude::*;

#[cfg(feature = "simulate")]
use crate::effects::InputSimulator;

#[allow(unused_imports)]
use crate::effects::{AudioEffect, Overdrive, StudioReverb};

pub const NUM_CHANNELS: usize = 2;

pub type Query<'a> = &'a Box<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut Box<dyn AudioEffect>;

#[derive(Debug, Lens, Clone)]
pub struct ChainData {
    pub chain: Arc<Chain>,
}

impl ChainData {
    pub fn as_mut<'a>(cx: &'a mut EventContext) -> &'a mut Chain {
        let chain = ChainData::chain.get(cx);
        unsafe {
            &mut *Arc::as_ptr(&chain).cast_mut()
        }
    }
}

impl Model for ChainData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        let chain = unsafe { &mut *Arc::as_ptr(&self.chain).cast_mut() };
        event.map(|event, _| match event {
            ChainCommand::Insert(audio_effect) => {
                chain.insert(audio_effect.clone());
            }
            ChainCommand::InsertAt(pos, audio_effect) => {
                chain.insert_at(*pos, audio_effect.clone());
            }
            ChainCommand::Remove(effect) => {
                chain.remove(*effect);
            }
            ChainCommand::Swap(e1, e2) => {
                if chain.effects.get(*e1).is_some() && chain.effects.get(*e2).is_some() {
                    chain.effects.swap(*e1, *e2);
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub enum ChainCommand {
    Insert(Box<dyn AudioEffect>),
    InsertAt(usize, Box<dyn AudioEffect>),
    Remove(usize),
    Swap(usize, usize),
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub effects: Vec<Box<dyn AudioEffect>>,
}

impl Chain {
    #[inline]
    pub fn process(&mut self, buffer: &mut [&mut [f32]]) {
        unsafe {
            let left: *mut &mut [f32] = std::mem::transmute(&mut buffer[0]);
            let right: *mut &mut [f32] = std::mem::transmute(&mut buffer[1]);
            self.effects
                .iter_mut()
                .for_each(|e| e.process((&mut *left, &mut *right)));
        }
    }

    #[inline]
    pub fn insert(&mut self, audio_effect: Box<dyn AudioEffect>) -> usize {
        self.effects.push(audio_effect);
        self.effects
            .clone()
            .into_iter()
            .enumerate()
            .last()
            .unwrap()
            .0
    }

    #[inline]
    pub fn insert_at(&mut self, index: usize, audio_effect: Box<dyn AudioEffect>) {
        self.effects.insert(index, audio_effect);
    }

    #[inline]
    pub fn remove(&mut self, index: usize) {
        if self.effects.get(index).is_some() {
            self.effects.remove(index);
        }
    }

    #[inline]
    pub fn query(&self, effect: usize) -> Option<Query> {
        let c = self.effects.get(effect)?;

        Some(c)
    }

    #[inline]
    pub fn query_mut(&mut self, effect: usize) -> Option<QueryMut> {
        let c = self.effects.get_mut(effect)?;
        Some(c)
    }

    #[inline]
    pub fn query_cast<T: AudioEffect + 'static>(&self, effect: usize) -> Option<&T> {
        self.effects.get(effect)?.as_any().downcast_ref::<T>()
    }

    #[inline]
    pub fn query_cast_mut<T: AudioEffect + 'static>(&mut self, effect: usize) -> Option<&mut T> {
        self.effects
            .get_mut(effect)?
            .as_any_mut()
            .downcast_mut::<T>()
    }

    #[inline]
    pub fn check(&self, effect: usize) -> bool {
        match self.query(effect) {
            Some(_) => true,
            None => false,
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut chain = Chain { effects: vec![] };

        #[cfg(feature = "simulate")]
        {
            chain.insert(Box::new(InputSimulator::default()));
        }

        chain
    }
}
