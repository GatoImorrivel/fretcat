use fretcat_effects:: Overdrive;
use nih_plug_vizia::vizia::{input::MouseState, prelude::*};

use crate::{effects::EffectHandle, EditorData};

use super::{CardData, CardEvent};

pub struct EffectList;

impl EffectList {
    pub fn new(cx: &mut Context) {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                Binding::new(
                    cx,
                    EditorData::chain.map(|c| c.borrow().effects.len()),
                    |cx, len| {
                        let chain = EditorData::chain.get(cx);
                        let borrow = chain.borrow();

                        for (i, effect) in borrow.effects.iter().enumerate() {
                            let data = borrow.query(effect).unwrap();

                            // default
                            let mut height = 200.0;

                            VStack::new(cx, |cx| {

                                if data.is::<Overdrive>() {
                                    let data = *data.clone().downcast_ref::<Overdrive>().unwrap();
                                    height = 200.0;

                                    EffectHandle::<Overdrive>::new(
                                        cx,
                                        chain.clone(),
                                        effect,
                                        &data,
                                    );
                                }
                            })
                            .width(Percentage(100.0))
                            .height(Pixels(height))
                            .on_drop(move |ex, _| {
                                let index = calculate_effect_index(i, ex.mouse(), ex.bounds());

                                let card = CardData::dragging.get(ex);

                                if let Some(card) = card {
                                    EditorData::chain.get(ex).borrow().add_to_queue(
                                        fretcat_effects::ChainCommand::InsertAt(
                                            index,
                                            card.spawn()
                                        ),
                                    );
                                    ex.emit(CardEvent::DragChange(None));
                                }
                            });
                        }
                    },
                );
            });
        });
    }
}

impl View for EffectList {
    fn element(&self) -> Option<&'static str> {
        Some("effect-list")
    }

    fn event(
        &mut self,
        cx: &mut nih_plug_vizia::vizia::prelude::EventContext,
        event: &mut nih_plug_vizia::vizia::prelude::Event,
    ) {
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
