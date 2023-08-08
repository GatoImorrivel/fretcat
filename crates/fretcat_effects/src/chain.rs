use std::{any::TypeId, collections::HashMap, hash::Hash, sync::Arc};

use crossbeam::queue::ArrayQueue;

use crate::{
    effect::{AudioEffect, Effect},
    overdrive::Overdrive,
};

pub type Query<'a> = &'a Box<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut Box<dyn AudioEffect>;

#[derive(Debug, Clone)]
pub enum ChainCommand {
    Insert(Box<dyn AudioEffect>),
    InsertAt(usize, Box<dyn AudioEffect>),
    Remove(Effect),
    Swap(Effect, Effect),
}

#[derive(Debug)]
pub struct Chain {
    pub effects: Vec<Effect>,
    pub update_queue: ArrayQueue<ChainCommand>,
    pub(crate) data_cache: HashMap<Effect, Box<dyn AudioEffect>>,
}

impl Chain {
    pub fn add_to_queue(&self, command: ChainCommand) {
        self.update_queue.force_push(command);
    }

    pub fn handle_command(&mut self, command: ChainCommand) {
        match command {
            ChainCommand::Insert(audio_effect) => {
                self.insert(audio_effect);
            }
            ChainCommand::InsertAt(pos, audio_effect) => {
                self.insert_at(pos, audio_effect);
            }
            ChainCommand::Remove(effect) => {
                self.remove(&effect);
            }
            ChainCommand::Swap(e1, e2) => {
                let i1 = self.index_of(&e1);
                let i2 = self.index_of(&e2);

                if let Some(i1) = i1 {
                    if let Some(i2) = i2 {
                        self.effects.swap(i1, i2);
                    }
                }
            }
        }
    }

    fn index_of(&self, effect: &Effect) -> Option<usize> {
        let (index, _) = self
            .effects
            .iter()
            .enumerate()
            .find(|(_, e)| **e == *effect)?;

        Some(index)
    }

    pub fn insert(&mut self, audio_effect: Box<dyn AudioEffect>) -> (usize, Effect) {
        let e = Effect::new();
        self.effects.push(e);
        self.data_cache.insert(e, audio_effect);
        self.effects.clone().into_iter().enumerate().last().unwrap()
    }

    pub fn insert_at(&mut self, index: usize, audio_effect: Box<dyn AudioEffect>) -> Effect {
        let e = Effect::new();
        self.effects.insert(index, e);
        self.data_cache.insert(e, audio_effect);
        self.effects.clone()[index]
    }

    pub fn remove(&mut self, effect: &Effect) {
        let fetch = self
            .effects
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_i, e)| e == effect);

        if let Some((index, effect)) = fetch {
            self.effects.remove(index);
            self.data_cache.remove(&effect);
        }
    }

    pub fn query(&self, effect: &Effect) -> Option<Query> {
        let c = self.data_cache.get(&effect)?;

        Some(c)
    }

    pub fn query_mut(&mut self, effect: &Effect) -> Option<QueryMut> {
        let c = self.data_cache.get_mut(effect)?;
        Some(c)
    }

    pub fn query_cast<T: AudioEffect + 'static>(&self, effect: &Effect) -> Option<&T> {
        self.data_cache.get(effect)?.as_any().downcast_ref::<T>()
    }

    pub fn query_cast_mut<T: AudioEffect + 'static>(&mut self, effect: &Effect) -> Option<&mut T> {
        self.data_cache
            .get_mut(effect)?
            .as_any_mut()
            .downcast_mut::<T>()
    }

    pub fn query_index(&self, index: usize) -> Option<(Effect, &Box<dyn AudioEffect>)> {
        let e = match self.effects.get(index) {
            Some(e) => e,
            None => return None,
        };

        let data = self.data_cache.get(e).unwrap();
        Some((*e, data))
    }

    pub fn get_position(&self, effect: &Effect) -> Option<usize> {
        self.effects.clone().into_iter().position(|e| e == *effect)
    }
}

impl Default for Chain {
    fn default() -> Self {
        let mut chain = Chain {
            effects: vec![],
            data_cache: HashMap::new(),
            update_queue: ArrayQueue::new(20).into(),
        };

        chain.insert(Box::new(Overdrive::default()));

        chain
    }
}
