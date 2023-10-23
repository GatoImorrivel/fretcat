use nih_plug::nih_log;
use nih_plug::vizia::prelude::*;

use super::{effect_handle::EffectHandle, CardSystem, CardEvent};
use fretcat_effects::{ChainCommand, ChainData};

#[derive(Debug, Lens, Clone, Copy)]
pub struct EffectList {
    pub dragging: Option<usize>,
    pub update_counter: u64
}

pub enum EffectListEvent {
    DragChange(Option<usize>),
}

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self { dragging: None, update_counter: 0 }.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, false, |cx| {
                Binding::new(
                    cx,
                    EffectList::update_counter,
                    |cx, _| {
                        let chain = ChainData::chain.get(cx);

                        for (index, effect) in chain.effects.iter().enumerate() {
                            VStack::new(cx, |cx| {
                                EffectHandle::new(cx, effect.clone(), index);
                            }).height(Pixels(effect.height()));
                        }
                        VStack::new(cx, |cx| {
                            VStack::new(cx, |cx| {
                                Label::new(cx, "+");
                            });
                        })
                        .class("new-effect-indicator")
                        .on_drop(|ex, _| {
                            let card = CardSystem::dragging.get(ex);

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

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|drag_event, _| match drag_event {
            EffectListEvent::DragChange(effect) => {
                self.dragging = effect.clone();
            }
        });

        event.map::<ChainCommand, _>(|event, _| match event {
            _ => self.update_counter += 1
        });
    }
}
