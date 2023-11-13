use nih_plug::vizia::prelude::*;

use crate::common::{EffectKind, EFFECT_CARDS, LIST_SPACING_UNITS};

#[derive(Debug, Data, Clone, Lens)]
pub struct CardList {
    pub(crate) current_kind: EffectKind,
    pub(crate) kinds: Vec<EffectKind>,
}

pub enum CardListMessage {
    ChangeKind(EffectKind),
}

impl CardList {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            current_kind: EffectKind::Distortion,
            kinds: EffectKind::variants(),
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                let kinds = Self::kinds.get(cx);
                let rows = kinds
                    .chunks(2)
                    .map(|chunk: &[EffectKind]| chunk.to_vec())
                    .collect::<Vec<_>>();

                for row in rows {
                    HStack::new(cx, move |cx| {
                        for kind in row {
                            Button::new(
                                cx,
                                move |ex| ex.emit(CardListMessage::ChangeKind(kind.clone())),
                                move |cx| Label::new(cx, &kind.to_string()),
                            )
                            .class("kind-btn")
                            .toggle_class(
                                "kind-selected-btn",
                                Self::current_kind.map(move |tab| *tab == kind),
                            );
                        }
                    })
                    .class("kind-btn-row");
                }
            })
            .height(Percentage(15.0))
            .class("kind-btn-wrapper");

            Binding::new(cx, Self::current_kind, |cx, bind| {
                ScrollView::new(cx, 0.0, 0.0, false, false, move |cx| {
                    let cards = EFFECT_CARDS.get(&bind.get(cx)).unwrap();

                    for card in cards {
                        card.render(cx);
                        Element::new(cx).height(LIST_SPACING_UNITS);
                    }
                });
            });
        })
    }
}

impl View for CardList {
    fn element(&self) -> Option<&'static str> {
        Some("card-list")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            CardListMessage::ChangeKind(val) => {
                self.current_kind = *val;
            }
        });
    }
}
