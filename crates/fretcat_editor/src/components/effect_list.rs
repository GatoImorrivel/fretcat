use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, ChainCommand, Effect, Overdrive};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::{image::Pixel, input::MouseState, prelude::*};

use crate::{
    effects::{EffectHandle, OverdriveControl},
    EditorData,
};

use super::{CardData, CardEvent};

#[derive(Debug, Lens, Clone, Copy)]
pub struct EffectList {
    pub dragging: Option<Effect>,
}

pub enum EffectListEvent {
    DragChange(Option<Effect>),
}

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self { dragging: None }.build(cx, |cx| {
            cx.add_stylesheet(include_str!("../../css/effect-list.css"))
                .unwrap();
            cx.add_stylesheet(include_str!("../../css/effect-handle.css"))
                .unwrap();
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                Binding::new(
                    cx,
                    EditorData::chain.map(|c| c.borrow().update_queue.len()),
                    |cx, _len| {
                        let chain = EditorData::chain.get(cx);
                        let borrow = chain.borrow();

                        for effect in borrow.effects.iter() {
                            let data = match borrow.query(effect) {
                                Some(data) => data,
                                None => continue
                            };

                            if data.is::<Overdrive>() {
                                EffectHandle::<Overdrive, OverdriveControl>::new(
                                    cx,
                                    effect.clone(),
                                    chain.clone(),
                                );
                            }
                        }
                        VStack::new(cx, |cx| {
                            VStack::new(cx, |cx| {
                                Label::new(cx, "+");
                            });
                        })
                        .class("new-effect-indicator")
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

        event.map(|drag_event, _| match drag_event {
            EffectListEvent::DragChange(effect) => {
                self.dragging = effect.clone();
            }
        });

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
                ChainCommand::Swap(e1, e2) => {
                    chain.borrow().add_to_queue(ChainCommand::Swap(e1, e2));
                }
            }
        }
    }
}
