use fretcat_effects::{AudioEffect, Effect, Overdrive};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::{EditorData, effects::EffectHandle};


pub struct EffectList;

pub enum EffectListMessage {
    Insert(usize, Box<dyn AudioEffect>),
    Remove(Effect),
}

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self {}.build(cx, |cx| {
            Binding::new(
                cx,
                EditorData::chain.map(|c| c.borrow().effects.len()),
                |cx, len| {
                    let chain = EditorData::chain.get(cx);
                    let borrow = chain.borrow();

                    for effect in borrow.effects.iter() {
                        let data = borrow.query(effect).unwrap();

                        if data.is::<Overdrive>() {
                            let data = *data.clone().downcast_ref::<Overdrive>().unwrap();

                            EffectHandle::<Overdrive>::new(cx, chain.clone(), effect, &data);
                        }
                    }
                },
            );
        });
    }
}

impl View for EffectList {
    fn element(&self) -> Option<&'static str> {
        Some("effect-list")
    }

    fn event(
        &mut self,
        cx: &mut nih_plug_vizia::vizia::prelude::EventContext,
        event: &mut nih_plug_vizia::vizia::prelude::Event,
    ) {

    }
}
