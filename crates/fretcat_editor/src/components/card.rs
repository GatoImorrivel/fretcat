use std::collections::HashMap;

use fretcat_effects::effects::{AudioEffect, Fuzz, Overdrive};
use nih_plug_vizia::vizia::prelude::*;

use crate::EffectKind;

lazy_static::lazy_static! {
    pub static ref EFFECT_CARDS: HashMap<EffectKind, Vec<Card>> = {
        let mut hashmap: HashMap<EffectKind, Vec<Card>> = HashMap::new();
        hashmap.insert(EffectKind::Distortion, vec![
            OVERDRIVE_CARD,
            FUZZ_CARD,
            DISTORTION_CARD
        ]);

        hashmap.insert(EffectKind::Delay, vec![]);
        hashmap.insert(EffectKind::Dynamics, vec![]);
        hashmap.insert(EffectKind::Echo, vec![]);
        hashmap.insert(EffectKind::Reverb, vec![]);

        hashmap
    };
}

pub fn card_system_init(cx: &mut Context) {
    CardData {
        dragging: None,
        cursor: (0.0, 0.0),
    }
    .build(cx);
}

pub fn card_system_view(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../../css/cards.css"))
        .unwrap();

    Binding::new(cx, CardData::dragging, |cx, bind| {
        let dragging = bind.get(cx);
        if let Some(dragging) = dragging {
            Binding::new(cx, CardData::cursor, move |cx, bind| {
                let cursor = bind.get(cx);
                VStack::new(cx, |cx| {
                    (dragging.content)(cx);
                })
                .class("card-base")
                .width(Pixels(300.0))
                .position_type(PositionType::SelfDirected)
                .left(Pixels(cursor.0))
                .top(Pixels(cursor.1));
            });
        }
    });
}

#[derive(Lens, Clone, PartialEq, Data)]
pub struct CardData {
    pub(crate) dragging: Option<Card>,
    pub(crate) cursor: (f32, f32),
}

pub enum CardEvent {
    DragChange(Option<Card>),
}

impl Model for CardData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseMove(x, y) => {
                self.cursor = (*x, *y);
            }
            WindowEvent::MouseUp(btn) => match btn {
                MouseButton::Left => {
                    self.dragging = None;
                }
                _ => {}
            },
            _ => {}
        });

        event.map(|e, _| match e {
            CardEvent::DragChange(card) => {
                self.dragging = card.clone();
            }
        });
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub(crate) content: fn(&mut Context),
    pub(crate) drag: fn(&mut EventContext),
    pub(crate) spawn: fn() -> Box<dyn AudioEffect>,
}

impl Data for Card {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Card {
    pub fn render(&self, cx: &mut Context) {
        VStack::new(cx, self.content)
            .on_drag(self.drag)
            .class("card-base");
    }

    pub fn content(&self, cx: &mut Context) {
        VStack::new(cx, self.content).class("card-base");
    }

    pub fn spawn(&self) -> Box<dyn AudioEffect> {
        (self.spawn)()
    }
}

pub const OVERDRIVE_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Drive")
                .font_family(vec![FamilyOwned::Name("Saturday".to_owned())])
                .color(Color::rgb(232, 86, 215))
                .font_size(75.0);
        })
        .border_width(Pixels(2.0))
        .border_color(Color::rgb(232, 86, 215))
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(OVERDRIVE_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Box::new(Overdrive::default()),
};

pub const FUZZ_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Fuzz")
                .font_family(vec![FamilyOwned::Name("Get Now".to_owned())])
                .color(Color::rgb(232, 142, 57))
                .font_size(75.0);
        })
        .border_width(Pixels(2.0))
        .border_color(Color::rgb(232, 142, 57))
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(FUZZ_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Box::new(Fuzz::default()),
};

pub const DISTORTION_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "DISTORTION")
                .font_family(vec![FamilyOwned::Name("Hatch".to_owned())])
                .color(Color::rgb(232, 57, 57))
                .font_size(40.0);
        })
        .border_width(Pixels(2.0))
        .border_color(Color::rgb(232, 57, 57))
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(DISTORTION_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Box::new(Fuzz::default()),
};