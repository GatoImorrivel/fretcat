use fretcat_effects::{AudioEffect, ChainCommand, ChainData, ChainHandle, Effect};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use crate::components::{CardData, CardEvent, EffectList, EffectListEvent};

#[derive(Debug, Clone)]
pub struct EffectHandle {
    handle: Box<dyn AudioEffect>,
    effect: Effect,
}

impl EffectHandle {
    pub fn new(cx: &mut Context, effect: Effect) {
        let chain = ChainData::chain.get(cx);
        let borrow = chain.borrow();
        let data = borrow.query(&effect).unwrap();
        let index = borrow.get_position(&effect).unwrap();
        Self {
            handle: data.clone(),
            effect: effect.clone(),
        }
        .build(cx, |cx| {
            HStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    Button::new(
                        cx,
                        move |ex| ex.emit(ChainCommand::Remove(effect.clone())),
                        |cx| Label::new(cx, ""),
                    )
                    .class("delete-effect-btn");
                    Element::new(cx);
                })
                .on_drag(move |ex| {
                    ex.emit(EffectListEvent::DragChange(Some(effect.clone())));
                    ex.set_drop_data(ex.current());
                })
                .class("effect-bar")
                .width(Stretch(3.0));

                Binding::new(
                    cx,
                    CardData::dragging.map(|drag| drag.is_some()),
                    move |cx, bind| {
                        let is_dragging = bind.get(cx);

                        if is_dragging {
                            Element::new(cx)
                                .position_type(PositionType::SelfDirected)
                                .width(Stretch(1.0))
                                .height(Percentage(50.0))
                                .on_drop(move |ex, _| {
                                    on_drop(ex, index as i32 - 1, effect.clone())
                                });
                            Element::new(cx)
                                .position_type(PositionType::SelfDirected)
                                .width(Stretch(1.0))
                                .height(Percentage(50.0))
                                .top(Percentage(50.0))
                                .on_drop(move |ex, _| {
                                    on_drop(ex, index as i32 + 1, effect.clone())
                                });
                        }
                    },
                );
            })
            .width(Stretch(1.0));
        });
    }
}

impl View for EffectHandle {
    fn element(&self) -> Option<&'static str> {
        Some("handle")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let chain = ChainData::chain.get(cx);
        self.handle.update(event, self.effect, chain);
    }
}

fn on_drop(ex: &mut EventContext, mut index: i32, effect: Effect) {
    let card = CardData::dragging.get(ex);
    let drag_effect = EffectList::dragging.get(ex);

    if index < 0 {
        index = 0;
    }

    if let Some(card) = card {
        ex.emit(ChainCommand::InsertAt(index as usize, card.spawn()));
        ex.emit(CardEvent::DragChange(None));
    }

    if let Some(drag_effect) = drag_effect {
        ex.emit(ChainCommand::Swap(effect, drag_effect));
        ex.emit(EffectListEvent::DragChange(None));
    }
}
