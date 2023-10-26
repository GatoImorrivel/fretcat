use std::sync::Arc;

use fretcat_effects::{effects::AudioEffect, ChainCommand, ChainData};
use nih_plug::vizia::prelude::*;

use crate::{
    components::{EffectList, EffectListEvent},
    systems::{CardEvent, CardSystem},
};

#[derive(Debug, Clone, Copy, Data)]
pub struct EffectHandle;

impl EffectHandle {
    pub fn new(cx: &mut Context, effect: Arc<dyn AudioEffect>, index: usize) -> Handle<Self> {
        Self.build(cx, |cx| {
            let height = effect.height();
            HStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    Button::new(
                        cx,
                        move |ex| ex.emit(ChainCommand::Remove(index)),
                        |cx| Label::new(cx, ""),
                    )
                    .class("delete-effect-btn")
                    .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                    Element::new(cx);
                })
                .on_drag(move |ex| {
                    ex.emit(EffectListEvent::DragChange(Some(index)));
                    ex.set_drop_data(ex.current());
                })
                .class("effect-bar")
                .height(Stretch(1.0))
                .width(Stretch(3.0));

                VStack::new(cx, move |cx| effect.view(cx, effect.clone()))
                    .width(Stretch(100.0))
                    .height(Stretch(1.0))
                    .on_drop(move |ex, _| on_drop(ex, index));

                Element::new(cx)
                    .position_type(PositionType::SelfDirected)
                    .width(Stretch(1.0))
                    .height(Percentage(50.0))
                    .visibility(CardSystem::is_dragging)
                    .on_drop(move |ex, _| on_drop(ex, index));
                Element::new(cx)
                    .position_type(PositionType::SelfDirected)
                    .width(Stretch(1.0))
                    .height(Percentage(50.0))
                    .top(Percentage(50.0))
                    .visibility(CardSystem::is_dragging)
                    .on_drop(move |ex, _| on_drop(ex, index + 1));
            })
            .class("effect-handle")
            .height(Pixels(height))
            .width(Stretch(1.0));
        })
    }
}

impl View for EffectHandle {
    fn element(&self) -> Option<&'static str> {
        Some("effect-handle")
    }
}

fn on_drop(ex: &mut EventContext, index: usize) {
    let card = CardSystem::dragging.get(ex);
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
