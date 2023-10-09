use fretcat_common::nih_plug::nih_log;
use fretcat_common::vizia::prelude::*;

use fretcat_effects::{ChainCommand, effects::Effect, ChainData};
use super::{CardData, CardEvent, effect_handle::EffectHandle};

#[derive(Debug, Lens, Clone, Copy)]
pub struct EffectList {
    pub dragging: Option<usize>,
}

pub enum EffectListEvent {
    DragChange(Option<usize>),
}

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self { dragging: None }.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, false, |cx| {
                Binding::new(
                    cx,
                    ChainData::chain.map(|c| c.borrow().update_queue.len()),
                    |cx, _len| {
                        let chain = ChainData::chain.get(cx);
                        let borrow = chain.borrow();

                        for (index, _) in borrow.effects.iter().enumerate() {
                            EffectHandle::new(cx, index).unwrap_or_else(|| {
                                nih_log!("dropped effect {:?}", index);
                            });
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
        cx: &mut EventContext,
        event: &mut Event,
    ) {
        let chain = ChainData::chain.get(cx);

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
