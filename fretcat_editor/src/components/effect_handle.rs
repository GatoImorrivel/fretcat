use fretcat_effects::{effects::AudioEffect, ChainCommand};
use nih_plug::vizia::prelude::*;

use crate::{
    components::{EffectList, EffectListEvent},
    systems::{CardEvent, CardSystem},
};

#[derive(Debug, Clone, Lens)]
pub struct EffectHandle {
    handle: fretcat_effects::prelude::EffectHandle<dyn AudioEffect>,
    active: bool
}

enum EffectHandleEvent {
    Toggle
}

impl EffectHandle {
    pub fn new(cx: &mut Context, effect: fretcat_effects::prelude::EffectHandle<dyn AudioEffect>, index: usize) -> Handle<Self> {
        Self {
            active: effect.active(),
            handle: effect.clone()
        }.build(cx, |cx| {
            HStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    Button::new(
                        cx,
                        move |ex| ex.emit(ChainCommand::Remove(index)),
                        |cx| Label::new(cx, ""),
                    )
                    .class("delete-effect-btn")
                    .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                    Button::new(
                        cx,
                        move |ex| ex.emit(EffectHandleEvent::Toggle),
                        |cx| Label::new(cx, Self::active.map(|active| if *active { "" } else { "" })),
                    )
                    .class("delete-effect-btn")
                    .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
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
                    .class("effect-container")
                    .on_drop(move |ex, _| on_drop(ex, index))
                    .disabled(Self::active.map(|active| !*active));

                Element::new(cx)
                    .position_type(PositionType::SelfDirected)
                    .width(Stretch(1.0))
                    .height(Percentage(50.0))
                    .display(CardSystem::is_dragging)
                    .on_drop(move |ex, _| on_drop(ex, index));
                Element::new(cx)
                    .position_type(PositionType::SelfDirected)
                    .width(Stretch(1.0))
                    .height(Percentage(50.0))
                    .top(Percentage(50.0))
                    .display(CardSystem::is_dragging)
                    .on_drop(move |ex, _| on_drop(ex, index + 1));
            })
            .class("effect-handle")
            .width(Stretch(1.0));
        }).overflow(Overflow::Hidden)
    }

    pub fn drag_handle(cx: &mut Context, effect: fretcat_effects::prelude::EffectHandle<dyn AudioEffect>) -> Handle<Self> {
        Self {
            active: effect.active(),
            handle: effect.clone()
        }.build(cx, |cx| {
            HStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    Button::new(
                        cx,
                        move |_| {},
                        |cx| Label::new(cx, ""),
                    )
                    .class("delete-effect-btn")
                    .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                    Button::new(
                        cx,
                        move |ex| ex.emit(EffectHandleEvent::Toggle),
                        |cx| Label::new(cx, Self::active.map(|active| if *active { "" } else { "" })),
                    )
                    .class("delete-effect-btn")
                    .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                })
                .class("effect-bar")
                .height(Stretch(1.0))
                .width(Stretch(3.0));

                VStack::new(cx, move |cx| effect.view(cx, effect.clone()))
                    .width(Stretch(100.0))
                    .height(Stretch(1.0))
                    .class("effect-container")
                    .disabled(true);
            })
            .class("effect-handle")
            .width(Stretch(1.0));
        }).overflow(Overflow::Hidden)
    }
}

impl View for EffectHandle {
    fn element(&self) -> Option<&'static str> {
        Some("effect-handle")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            EffectHandleEvent::Toggle => {
                self.handle.set_active(!self.handle.active());
                self.active = self.handle.active();
            }
        });
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
