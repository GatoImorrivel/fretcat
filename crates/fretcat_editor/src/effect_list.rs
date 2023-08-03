use fretcat_effects::{effect::{AudioEffect, Effect}, chain::{ChainHandle, ChainEvent}};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::{prelude::*, input::MouseState};
use std::marker::PhantomData;
use nih_plug_vizia::vizia::input::Code;

use crate::card::{CardData, CardEvent};

pub struct EffectList {
    p: PhantomData<ChainHandle>,
}

impl EffectList
{
    /// Creates a new List view with a binding to the given lens and a template for constructing the list items
    pub fn new<F>(cx: &mut Context, bar_height: f32, item: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, usize, Effect, &Box<dyn AudioEffect>) + Clone,
    {
        EffectList {
            p: PhantomData,
        }
        .build(cx, move |cx| {
            // Bind to the list data
            Binding::new(cx, ChainHandle::root.map(|lst| lst.effects.len()), move |cx, list_len| {
                // If the number of list items is different to the number of children of the ListView
                // then remove and rebuild all the children
                let list_len = list_len.get_fallible(cx).map_or(0, |d| d);
                let chain = ChainHandle::root.get(cx);

                for index in 0..list_len {
                    nih_log!("REDRAW");
                    let (effect, data) = chain.query_index(index).unwrap();
                    let i1 = item.clone();
                    VStack::new(cx, move |cx| {
                        (i1)(cx, index, effect, data);
                    })
                    .on_drop(move |ex, _| {
                        let index = calculate_effect_index(index, ex.mouse(), ex.bounds());

                        let card = CardData::dragging.get(ex);

                        if let Some(card) = card {
                            ex.emit(ChainEvent::Insert(card.spawn(), index));
                            ex.emit(CardEvent::DragChange(None));
                        }
                    })
                    .height(Pixels(data.height() + bar_height));
                }
            });
        })
    }
}

impl View for EffectList {
    fn element(&self) -> Option<&'static str> {
        Some("list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
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