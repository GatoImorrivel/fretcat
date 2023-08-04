use std::{collections::HashMap, sync::Arc};

use crossbeam::queue::ArrayQueue;

use crate::{
    effect::{AudioEffect, Effect},
    overdrive::Overdrive,
};

pub type Query<'a> = &'a Box<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut Box<dyn AudioEffect>;

#[derive(Debug, Clone)]
pub enum ChainCommand {
    Insert(Box<dyn AudioEffect>)
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub effects: Vec<Effect>,
    pub update_queue: Arc<ArrayQueue<ChainCommand>>,
    data_cache: HashMap<Effect, Box<dyn AudioEffect>>,
}

impl Chain {
    pub fn add_to_queue(&self, command: ChainCommand) {
        self.update_queue.force_push(command);
    }

    pub fn handle_command(&mut self, command: ChainCommand) {
        match command {
            ChainCommand::Insert(data) => {
                self.insert(data);
            }
        }
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
}

impl Default for Chain {
    fn default() -> Self {
        let mut chain = Chain {
            effects: vec![],
            data_cache: HashMap::new(),
            update_queue: Arc::new(ArrayQueue::new(5).into())
        };

        chain.insert(Box::new(Overdrive::default()));

        chain
    }
}
