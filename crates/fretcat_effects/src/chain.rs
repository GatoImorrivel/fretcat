use std::{sync::Arc, time::Instant};

use indexmap::IndexMap;
use nih_plug::{vizia::prelude::*, nih_log};

#[cfg(feature = "simulate")]
use crate::effects::InputSimulator;

#[allow(unused_imports)]
use crate::effects::{AudioEffect, Overdrive, StudioReverb};
use crate::effects::{PostFX, PreFX};

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
        unsafe { &mut *Arc::as_ptr(&chain).cast_mut() }
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
    pub pre_fx: IndexMap<PreFX, Box<dyn AudioEffect>>,
    pub post_fx: IndexMap<PostFX, Box<dyn AudioEffect>>,
    pub in_avg_amplitude: (f32, f32),
    pub out_avg_amplitude: (f32, f32),
    pub debug_clock: Instant
}

impl Chain {
    #[inline]
    pub fn process(&mut self, buffer: &mut [&mut [f32]]) {
        let d1 = self.debug_clock.elapsed();
        unsafe {
            let left: *mut &mut [f32] = std::mem::transmute(&mut buffer[0]);
            let right: *mut &mut [f32] = std::mem::transmute(&mut buffer[1]);

            self.in_avg_amplitude = Self::get_avg_amplitude((&*left, &*right));

            self.pre_fx
                .iter_mut()
                .for_each(|(_, fx)| fx.process((&mut *left, &mut *right)));
            self.effects
                .iter_mut()
                .for_each(|e| e.process((&mut *left, &mut *right)));
            self.post_fx
                .iter_mut()
                .for_each(|(_, fx)| fx.process((&mut *left, &mut *right)));

            self.out_avg_amplitude = Self::get_avg_amplitude((&*left, &*right));
        }
        let d2 = self.debug_clock.elapsed();

        nih_log!("{}", (d2 - d1).as_nanos());
    }

    #[inline]
    fn get_avg_amplitude(buffer: (&[f32], &[f32])) -> (f32, f32) {
        (buffer.0.iter().sum::<f32>() / buffer.0.len() as f32, buffer.1.iter().sum::<f32>() / buffer.1.len() as f32)
    }

    #[inline]
    pub fn get_pre_fx<T: AudioEffect>(&mut self, fx: &PreFX) -> Option<&mut T> {
        self.pre_fx.get_mut(fx)?.downcast_mut::<T>()
    }

    #[inline]
    pub fn get_post_fx<T: AudioEffect>(&mut self, fx: &PostFX) -> Option<&mut T> {
        self.post_fx.get_mut(fx)?.downcast_mut::<T>()
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
        let mut chain = Chain {
            effects: vec![],
            pre_fx: IndexMap::new(),
            post_fx: IndexMap::new(),
            in_avg_amplitude: (0.0, 0.0),
            out_avg_amplitude: (0.0, 0.0),
            debug_clock: Instant::now()
        };

        #[cfg(feature = "simulate")]
        {
            chain.insert(Box::new(InputSimulator::default()));
        }

        chain
    }
}
