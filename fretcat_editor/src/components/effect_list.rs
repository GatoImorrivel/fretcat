use std::sync::Arc;

use nih_plug::vizia::prelude::*;

use crate::{
    common::{LIST_SPACING, LIST_SPACING_UNITS},
    systems::{CardEvent, CardSystem},
};

use super::effect_handle::EffectHandle;
use fretcat_effects::{Chain, ChainCommand};

#[derive(Debug, Lens, Clone, Copy)]
pub struct EffectList {
    pub dragging: Option<usize>,
    pub update_counter: u64,
}

pub enum EffectListEvent {
    DragChange(Option<usize>),
}

impl EffectList {
    pub fn new<L: Lens<Target = Arc<Chain>>>(cx: &mut Context, lens: L) -> Handle<Self> {
        Self {
            dragging: None,
            update_counter: 0,
        }
        .build(cx, move |cx| {
            cx.add_listener(|view: &mut EffectList, cx, event| {
                event.map::<ChainCommand, _>(|_, _| {
                    view.update_counter += 1;
                });
            });

            ScrollView::new(cx, 0.0, 0.0, false, false, move |cx| {
                Binding::new(cx, EffectList::update_counter, move |cx, _| {
                    let chain = lens.get(cx);

                    for (index, effect) in chain.effects.iter().enumerate() {
                        VStack::new(cx, |cx| {
                            EffectHandle::new(cx, effect.clone(), index);
                        })
                        .height(Pixels(
                            effect.height()
                                + if effect.height() > 100.0 {
                                    LIST_SPACING
                                } else {
                                    0.0
                                },
                        ));
                        Element::new(cx).height(LIST_SPACING_UNITS);
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
                });
            })
            .width(Stretch(1.0))
            .height(Stretch(1.0))
            .overflow(Overflow::Hidden);
        })
        .overflow(Overflow::Hidden)
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
    }
}
