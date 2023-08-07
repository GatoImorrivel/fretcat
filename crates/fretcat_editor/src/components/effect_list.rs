use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, ChainCommand, Effect, Overdrive};
use nih_plug_vizia::vizia::{input::MouseState, prelude::*, image::Pixel};

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
                        VStack::new(cx, |cx| {
                            Label::new(cx, "");
                        })
                            .height(Pixels(200.0))
                            .width(Percentage(100.0))
                            .on_drop(|ex, _| {
                                let card = CardData::dragging.get(ex);

                                if let Some(card) = card {
                                    ex.emit(ChainCommand::Insert(card.spawn()));
                                    ex.emit(CardEvent::DragChange(None));
                                }
                            });
                        Element::new(cx)
                            .height(Stretch(1.0))
                            .width(Percentage(100.0))
                            .on_drop(|ex, _| {
                                let card = CardData::dragging.get(ex);

                                if let Some(card) = card {
                                    ex.emit(ChainCommand::Insert(card.spawn()));
                                    ex.emit(CardEvent::DragChange(None));
                                }
                            });
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
        let chain = EditorData::chain.get(cx);
        let event = event.take();
        if let Some(e) = event {
            match e {
                ChainCommand::Insert(data) => {
                    chain.borrow().add_to_queue(ChainCommand::Insert(data));
                }
                ChainCommand::InsertAt(index, data) => {
                    chain
                        .borrow()
                        .add_to_queue(ChainCommand::InsertAt(index, data));
                }
                ChainCommand::Remove(effect) => {
                    chain.borrow().add_to_queue(ChainCommand::Remove(effect));
                }
            }
        }
    }
}
