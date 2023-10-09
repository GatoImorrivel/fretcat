use std::{collections::HashMap, sync::Arc};

use atomic_refcell::AtomicRefCell;
use fretcat_common::vizia::prelude::*;

use crossbeam::queue::ArrayQueue;

use crate::effects::{AudioEffect, Effect, Overdrive};

pub type Query<'a> = &'a Box<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut Box<dyn AudioEffect>;
pub type ChainHandle = Arc<AtomicRefCell<Chain>>;

#[derive(Debug, Lens, Clone)]
pub struct ChainData {
    pub chain: ChainHandle,
}

impl Model for ChainData {}

#[derive(Debug, Clone)]
pub enum ChainCommand {
    Insert(Box<dyn AudioEffect>),
    InsertAt(usize, Box<dyn AudioEffect>),
    Remove(usize),
    Swap(usize, usize),
}

#[derive(Debug)]
pub struct Chain {
    pub effects: Vec<Box<dyn AudioEffect>>,
    pub update_queue: ArrayQueue<ChainCommand>,
}

impl Chain {
    #[inline]
    pub fn process(&self, buffer: &mut [f32]) {
        self.effects.iter().for_each(|e| {
            e.process(buffer);
        });
    }

    #[inline]
    pub fn add_to_queue(&self, command: ChainCommand) {
        self.update_queue.force_push(command);
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
            update_queue: ArrayQueue::new(20).into(),
        };

        chain.insert(Box::new(Overdrive::default()));

        chain
    }
}

impl Clone for Chain {
    fn clone(&self) -> Self {
        Self {
            effects: self.effects.clone(),
            update_queue: ArrayQueue::new(20),
        }
    }
}
