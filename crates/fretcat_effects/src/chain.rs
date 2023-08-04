use std::{collections::HashMap, hash::Hash, any::TypeId};

use crate::{effect::{Effect, AudioEffect}, overdrive::Overdrive};

pub type Query<'a> = (TypeId, &'a Box<dyn AudioEffect>);
pub type QueryMut<'a> = (TypeId, &'a mut Box<dyn AudioEffect>);

#[derive(Debug, Clone)]
pub struct Chain {
    pub effects: Vec<Effect>,
    data_cache: HashMap<Effect, Box<dyn AudioEffect>>,
    type_cache: HashMap<Effect, TypeId>
}

impl Chain {
    pub fn insert(&mut self, audio_effect: impl AudioEffect) -> (usize, Effect) {
        let audio_effect = Box::new(audio_effect);
        let type_id = audio_effect.type_id();
        let e = Effect::new();
        self.effects.push(e);
        self.data_cache.insert(e, audio_effect);
        self.type_cache.insert(e, type_id);
        self.effects.clone().into_iter().enumerate().last().unwrap()
    }

    pub fn insert_at(&mut self, index: usize, audio_effect: Box<dyn AudioEffect>) -> Effect {
        let e = Effect::new();
        self.effects.insert(index, e);
        self.type_cache.insert(e, audio_effect.type_id());
        self.data_cache.insert(e, audio_effect);
        self.effects.clone()[index]
    }

    pub fn remove(&mut self, effect: &Effect) {
        let fetch = self.effects.clone().into_iter().enumerate().find(|(_i, e)| {
            e == effect
        });

        if let Some((index, effect)) = fetch {
            self.effects.remove(index);
            self.data_cache.remove(&effect);
            self.type_cache.remove(&effect);
        }
    }

    pub fn get(&self, effect: &Effect) -> Option<&Box<dyn AudioEffect>> { 
        self.data_cache.get(effect)
    }

    pub fn get_mut(&mut self, effect: &Effect) -> Option<&mut Box<dyn AudioEffect>> { 
        self.data_cache.get_mut(effect)
    }

    pub fn query(&self, effect: &Effect) -> Option<Query> {
        let c = self.get(&effect)?;

        Some((self.type_cache.get(effect)?.clone(), c))
    }

    pub fn query_mut(&mut self, effect: &Effect) -> Option<QueryMut> {
        let t = self.type_cache.get(effect)?.clone();
        let c = self.get_mut(effect)?;
        Some((t, c))
    }

    pub fn query_cast<T: AudioEffect + 'static>(&self, effect: &Effect) -> Option<&T> {
        self.get(effect)?.as_any().downcast_ref::<T>()
    }

    pub fn query_cast_mut<T: AudioEffect + 'static>(&mut self, effect: &Effect) -> Option<&mut T> {
        self.get_mut(effect)?.as_any_mut().downcast_mut::<T>()
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
            type_cache: HashMap::new()
        };

        chain.insert(Overdrive::default());

        chain
    }
}