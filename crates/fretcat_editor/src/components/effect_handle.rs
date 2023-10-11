use fretcat_common::vizia::prelude::*;
use fretcat_effects::{ChainData, ChainCommand};

use crate::components::{CardData, CardEvent, EffectList, EffectListEvent};

#[derive(Debug, Clone)]
pub struct EffectHandle {
    index: usize,
}

impl EffectHandle {
    pub fn new(cx: &mut Context, effect: usize) -> Option<()> {
        let chain = ChainData::chain.get(cx);
        let data = chain.query(effect)?;
        HStack::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                Button::new(
                    cx,
                    move |ex| ex.emit(ChainCommand::Remove(effect)),
                    |cx| Label::new(cx, ""),
                )
                .class("delete-effect-btn")
                .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                Element::new(cx);
            })
            .on_drag(move |ex| {
                ex.emit(EffectListEvent::DragChange(Some(effect)));
                ex.set_drop_data(ex.current());
            })
            .class("effect-bar")
            .height(Stretch(1.0))
            .width(Stretch(3.0));

            VStack::new(cx, move |cx| {
                Self {
                    index: effect
                }
                .build(cx, |cx| {
                    data.view(cx, effect);
                });
            })
            .width(Stretch(100.0))
            .height(Stretch(1.0))
            .on_drop(move |ex, _| on_drop(ex, effect));

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
                            .on_drop(move |ex, _| on_drop(ex, effect));
                        Element::new(cx)
                            .position_type(PositionType::SelfDirected)
                            .width(Stretch(1.0))
                            .height(Percentage(50.0))
                            .top(Percentage(50.0))
                            .on_drop(move |ex, _| on_drop(ex, effect + 1));
                    }
                },
            );
        })
        .class("effect-handle")
        .height(Pixels(data.height()))
        .width(Stretch(1.0));

        Some(())
    }
}

impl View for EffectHandle {
    fn element(&self) -> Option<&'static str> {
        Some("effect-handle")        
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let chain = ChainData::as_mut(cx);
        
        if chain.effects.get(self.index).is_some() {
            chain.effects.get_mut(self.index).unwrap().update(event);
        }
    }
}

fn on_drop(ex: &mut EventContext, index: usize) {
    let card = CardData::dragging.get(ex);
    let drag_effect = EffectList::dragging.get(ex);

    if let Some(card) = card {
        ex.emit(ChainCommand::InsertAt(index, card.spawn()));
        ex.emit(CardEvent::DragChange(None));
    }

    if let Some(drag_effect) = drag_effect {
        ex.emit(ChainCommand::Swap(index, drag_effect));
        ex.emit(EffectListEvent::DragChange(None));
    }
}
