use std::sync::Arc;

use fretcat_common::vizia::prelude::*;

use crate::effects::{AudioEffect, Overdrive, StudioReverb};

pub type Query<'a> = &'a Box<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut Box<dyn AudioEffect>;

#[derive(Debug, Lens, Clone)]
pub struct ChainData {
    pub chain: Arc<Chain>,
}

impl Model for ChainData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let event = event.take();
        let chain = unsafe {
            &mut *Arc::as_ptr(&self.chain).cast_mut()
        };
        if let Some(e) = event {
            match e {
                ChainCommand::Insert(data) => {
                    chain.handle_command(ChainCommand::Insert(data));
                }
                ChainCommand::InsertAt(index, data) => {
                    chain.handle_command(ChainCommand::InsertAt(index, data));
                }
                ChainCommand::Remove(index) => {
                    chain.handle_command(ChainCommand::Remove(index));
                }
                ChainCommand::Swap(e1, e2) => {
                    chain.handle_command(ChainCommand::Swap(e1, e2));
                }
            }
        }
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
    pub fn process(&mut self, buffer: &mut [f32]) {
        self.effects.iter_mut().for_each(|e| {
            e.process(buffer);
        });
    }

    #[inline]
    pub fn handle_command(&mut self, command: ChainCommand) {
        match command {
            ChainCommand::Insert(audio_effect) => {
                self.insert(audio_effect);
            }
            ChainCommand::InsertAt(pos, audio_effect) => {
                self.insert_at(pos, audio_effect);
            }
            ChainCommand::Remove(effect) => {
                self.remove(effect);
            }
            ChainCommand::Swap(e1, e2) => {
                if self.effects.get(e1).is_some() && self.effects.get(e2).is_some() {
                    self.effects.swap(e1, e2);
                }
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, audio_effect: Box<dyn AudioEffect>) -> usize {
        self.effects.push(audio_effect);
        self.effects.clone().into_iter().enumerate().last().unwrap().0
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
        let mut chain = Chain {
            effects: vec![],
        };

        chain.insert(Box::new(StudioReverb::default()));

        chain
    }
}