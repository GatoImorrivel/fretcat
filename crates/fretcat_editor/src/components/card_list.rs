use nih_plug::vizia::prelude::*;

use crate::common::{EffectKind, EFFECT_CARDS};

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
                            );
                        }
                    });
                }
            });

            VStack::new(cx, |cx| {
                let cards = EFFECT_CARDS.get(&Self::current_kind.get(cx)).unwrap();

                for card in cards {
                    card.render(cx);
                }
            });
        })
    }
}

impl View for CardList {
    fn element(&self) -> Option<&'static str> {
        Some("card-list")
    }
}
