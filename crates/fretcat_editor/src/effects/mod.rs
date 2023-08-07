use std::{
    marker::PhantomData,
    sync::{Arc, Condvar},
};

use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, ChainCommand, Effect};
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

mod overdrive;
pub use overdrive::OverdriveControl;

use crate::{
    components::{CardData, CardEvent},
    EditorData,
};

const EFFECT_BAR_HEIGHT: f32 = 30.0;

pub trait Control<T: AudioEffect>: View {
    type Message;
    fn init(data: &T) -> Self;
    fn view(cx: &mut Context);
    fn update(event: &Self::Message, data: &mut T);
    fn height() -> f32;
}

#[derive(Debug, Clone)]
pub struct EffectHandle<T: AudioEffect, C: Control<T>> {
    effect: Effect,
    chain: Arc<AtomicRefCell<Chain>>,
    p: PhantomData<T>,
    c: PhantomData<C>,
}

impl<T, C> EffectHandle<T, C>
where
    T: AudioEffect,
    C: Control<T>,
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
            Button::new(
                cx,
                move |ex| ex.emit(ChainCommand::Remove(effect.clone())),
                |cx| Label::new(cx, "deletar"),
            );
        })
        .width(Percentage(100.0))
        .height(Pixels(EFFECT_BAR_HEIGHT));
        control.build(cx, |cx| {
            handle.build(cx);

            VStack::new(cx, move |cx| {
                C::view(cx);
            })
            .width(Percentage(100.0))
            .height(Pixels(C::height()))
            .on_drop(move |ex, _| {
                let index = calculate_effect_index(index, ex.mouse(), ex.bounds());

                let card = CardData::dragging.get(ex);

                if let Some(card) = card {
                    ex.emit(ChainCommand::InsertAt(index, card.spawn()));
                    ex.emit(CardEvent::DragChange(None));
                }
            });
        });
    }
}

impl<T, C> Model for EffectHandle<T, C>
where
    T: AudioEffect,
    C: Control<T>,
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

fn calculate_effect_index(i: usize, mouse: &MouseState<Entity>, bounds: BoundingBox) -> usize {
    let middle_point = (bounds.y + bounds.h) / 2.0;

    if mouse.cursory < middle_point {
        if !i <= 0 {
            i - 1
        } else {
            i
        }
    } else {
        i + 1
    }
}
