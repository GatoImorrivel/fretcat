use std::sync::Arc;

use indexmap::IndexMap;

use crate::effects::{Mono, Gain, NoiseGate};

pub use super::prelude::*;

pub const NUM_CHANNELS: usize = 2;

pub type Query<'a> = &'a EffectHandle<dyn AudioEffect>;
pub type QueryMut<'a> = &'a mut EffectHandle<dyn AudioEffect>;


#[derive(Debug, Lens, Clone)]
pub struct ChainData {
    pub chain: Arc<Chain>,
}

impl ChainData {
    pub fn as_mut_ex<'a>(cx: &'a mut EventContext) -> &'a mut Chain {
        let chain = ChainData::chain.get(cx);
        unsafe { &mut *Arc::as_ptr(&chain).cast_mut() }
    }

    pub fn as_mut_cx<'a>(cx: &'a mut Context) -> &'a mut Chain {
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
            ChainCommand::Clear => {
                chain.effects.clear();
            }
            ChainCommand::Load(effects) => {
                chain.load(effects.clone());
            }
        });
    }
}

#[derive(Debug, Clone)]
pub enum ChainCommand {
    Insert(Arc<dyn AudioEffect>),
    InsertAt(usize, Arc<dyn AudioEffect>),
    Remove(usize),
    Swap(usize, usize),
    Load(Vec<Arc<dyn AudioEffect>>),
    Clear,
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub effects: Vec<EffectHandle<dyn AudioEffect>>,
    pub pre_fx: IndexMap<PreFX, Box<dyn AudioEffect>>,
    pub post_fx: IndexMap<PostFX, Box<dyn AudioEffect>>,
    pub in_avg_amplitude: (f32, f32),
    pub out_avg_amplitude: (f32, f32),
}

impl Chain {
    #[inline]
    pub fn process<'a>(&mut self, buffer: &mut [&'a mut [f32]], transport: &nih_plug::prelude::Transport) {
            let mut frame = Frame::from(buffer);

            self.pre_fx
                .iter_mut()
                .for_each(|(_, fx)| fx.process(&mut frame, transport));

            self.in_avg_amplitude = Self::get_rms(&frame);

            self.effects.iter_mut().for_each(|e| {
                e.process_if_active(&mut frame, transport)
            });

            self.post_fx
                .iter_mut()
                .for_each(|(_, fx)| fx.process(&mut frame, transport));

            self.out_avg_amplitude = Self::get_rms(&frame);
    }

    #[inline]
    fn get_rms(frame: &Frame) -> (f32, f32) {
        (
            gain_to_db_fast(rms(&frame.get_left())),
            gain_to_db_fast(rms(&frame.get_right())),
        )
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
    pub fn load(&mut self, effects: Vec<Arc<dyn AudioEffect>>) {
        self.effects = effects.into_iter().map(|effect| EffectHandle::from(effect)).collect::<Vec<_>>();
    }

    #[inline]
    pub fn insert(&mut self, audio_effect: Arc<dyn AudioEffect>) -> usize {
        self.effects.push(EffectHandle::from(audio_effect));
        self.effects
            .clone()
            .into_iter()
            .enumerate()
            .last()
            .unwrap()
            .0
    }

    #[inline]
    pub fn insert_at(&mut self, index: usize, audio_effect: Arc<dyn AudioEffect>) {
        self.effects.insert(index, EffectHandle::from(audio_effect));
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
        let effect = self.effects.get(effect)?;
        effect.as_any().downcast_ref::<T>()
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
        };

        chain
            .pre_fx
            .insert(PreFX("mono"), Box::new(Mono::default()));
        chain
            .pre_fx
            .insert(PreFX("in_gain"), Box::new(Gain::default()));
        chain
            .pre_fx
            .insert(PreFX("noise_gate"), Box::new(NoiseGate::default()));


        chain
            .post_fx
            .insert(PostFX("out_gain"), Box::new(Gain::default()));

        chain
    }
}
