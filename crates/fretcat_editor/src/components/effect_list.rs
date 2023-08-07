use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, Effect, Overdrive};
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

use crate::{
    effects::{EffectHandle, OverdriveControl},
    EditorData,
};

use super::{CardData, CardEvent};

pub struct EffectList;

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                Binding::new(
                    cx,
                    EditorData::chain.map(|c| c.borrow().effects.len()),
                    |cx, _len| {
                        let chain = EditorData::chain.get(cx);
                        let borrow = chain.borrow();

                        for effect in borrow.effects.iter() {
                            let data = borrow.query(effect).unwrap();

                            if data.is::<Overdrive>() {
                                EffectHandle::<Overdrive, OverdriveControl>::new(
                                    cx,
                                    effect.clone(),
                                    chain.clone(),
                                );
                            }
                        }
                    },
                );
            });
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
