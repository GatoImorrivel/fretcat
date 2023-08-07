use std::{
    marker::PhantomData,
    sync::{Arc, Condvar},
};

use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, Effect};
use nih_plug_vizia::vizia::{prelude::*, input::MouseState};

mod overdrive;
pub use overdrive::OverdriveControl;

use crate::{EditorData, components::{CardData, CardEvent}};

pub trait Control<T: AudioEffect>: View {
    type Message;
    fn init(data: &T) -> Self;
    fn view(cx: &mut Context);
    fn update(event: &Self::Message, data: &mut T);
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
            effect: effect,
            p: PhantomData,
            c: PhantomData,
        };

        control.build(cx, |cx| {
            handle.build(cx);

            VStack::new(cx, |cx| {
                C::view(cx);
            })
                .width(Percentage(100.0))
                .height(Pixels(200.0))
                .on_drop(move |ex, _| {
                    let index = calculate_effect_index(index, ex.mouse(), ex.bounds());

                    let card = CardData::dragging.get(ex);

                    if let Some(card) = card {
                        EditorData::chain.get(ex).borrow().add_to_queue(
                            fretcat_effects::ChainCommand::InsertAt(index, card.spawn()),
                        );
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