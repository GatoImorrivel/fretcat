use fretcat_effects::{effect::{AudioEffect, Effect}, chain::ChainHandle};
use nih_plug_vizia::vizia::prelude::*;
use std::marker::PhantomData;
use nih_plug_vizia::vizia::input::Code;

pub struct EffectList {
    p: PhantomData<ChainHandle>,
}

impl EffectList
{
    /// Creates a new List view with a binding to the given lens and a template for constructing the list items
    pub fn new<F>(cx: &mut Context, item: F) -> Handle<Self>
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
                    let (effect, data) = chain.query_index(index).unwrap();
                    let i1 = item.clone();
                    VStack::new(cx, move |cx| {
                        (i1)(cx, index, effect, data);
                    })
                    .height(Pixels(data.height()));
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