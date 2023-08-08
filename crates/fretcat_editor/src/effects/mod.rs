use std::{
    marker::PhantomData,
    sync::{Arc, Condvar},
};

use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, ChainCommand, Effect};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

mod overdrive;
pub use overdrive::OverdriveControl;

use crate::{
    components::{CardData, CardEvent, EffectList, EffectListEvent},
    EditorData,
};

pub trait Control: View {
    type Message;
    type Target: AudioEffect;
    fn init(data: &Self::Target) -> Self;
    fn view(cx: &mut Context);
    fn update(event: &Self::Message, data: &mut Self::Target);
    fn height() -> f32;
}

#[derive(Debug, Clone)]
pub struct EffectHandle<T: AudioEffect, C: Control<Target = T>> {
    effect: Effect,
    chain: Arc<AtomicRefCell<Chain>>,
    p: PhantomData<T>,
    c: PhantomData<C>,
}

impl<T, C> EffectHandle<T, C>
where
    T: AudioEffect,
    C: Control<Target = T>,
    C::Message: Send,
{
    pub fn new(cx: &mut Context, effect: Effect, chain: Arc<AtomicRefCell<Chain>>) {
        let borrow = chain.borrow();
        let data = borrow.query_cast::<T>(&effect).unwrap();
        let control = C::init(data);
        let index = borrow.get_position(&effect).unwrap();
        let handle = Self {
            chain: chain.clone(),
            effect: effect.clone(),
            p: PhantomData,
            c: PhantomData,
        };

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

            control
                .build(cx, |cx| {
                    handle.build(cx);
                    VStack::new(cx, move |cx| {
                        C::view(cx);
                    });
                })
                .width(Stretch(100.0));

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
                            .on_drop(move |ex, _| on_drop(ex, index as i32 - 1, effect.clone()));
                        Element::new(cx)
                            .position_type(PositionType::SelfDirected)
                            .width(Stretch(1.0))
                            .height(Percentage(50.0))
                            .top(Percentage(50.0))
                            .on_drop(move |ex, _| on_drop(ex, index as i32 + 1, effect.clone()));
                    }
                },
            );
        })
        .width(Stretch(1.0))
        .height(Pixels(C::height()));
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

impl<T, C> Model for EffectHandle<T, C>
where
    T: AudioEffect,
    C: Control<Target = T>,
    C::Message: Send,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event: &C::Message, _| {
            let chain = unsafe { self.chain.as_ptr().as_mut().unwrap() };
            let data = chain.query_cast_mut::<T>(&self.effect).unwrap();
            C::update(event, data);
        });
    }
}
