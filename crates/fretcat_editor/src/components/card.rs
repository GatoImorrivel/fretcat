use std::{collections::HashMap, sync::Arc};

use fretcat_effects::effects::{AudioEffect, Fuzz, Overdrive, StudioReverb};
use nih_plug::vizia::prelude::*;

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
        hashmap.insert(EffectKind::Reverb, vec![
            REVERB_CARD
        ]);

        hashmap
    };
}

#[derive(Lens, Clone, PartialEq, Data)]
pub struct CardSystem {
    pub(crate) dragging: Option<Card>,
    pub(crate) is_dragging: bool,
    pub(crate) cursor: (f32, f32),
}

impl CardSystem {
    pub fn init(cx: &mut Context) {
        Self {
            dragging: None,
            is_dragging: false,
            cursor: (0.0, 0.0),
        }
        .build(cx);
    }

    pub fn view(cx: &mut Context) {
        Binding::new(cx, CardSystem::is_dragging, |cx, bind| {
            let is_dragging = bind.get(cx);
            if is_dragging {
                let card = CardSystem::dragging.get(cx);
                VStack::new(cx, |cx| {
                    if let Some(card) = card {
                        card.content(cx);
                    }
                })
                .background_color(Color::blue())
                .class("card-base")
                .width(Pixels(300.0))
                .position_type(PositionType::SelfDirected)
                .left(CardSystem::cursor.map(|cursor| Pixels(cursor.0)))
                .top(CardSystem::cursor.map(|cursor| Pixels(cursor.1)));
            }
        });
    }
}

pub enum CardEvent {
    DragChange(Option<Card>),
}

impl Model for CardSystem {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseMove(x, y) => {
                if !x.is_nan() && !y.is_nan() {
                    self.cursor = (*x / cx.scale_factor(), *y / cx.scale_factor());
                }
            }
            WindowEvent::MouseUp(btn) => match btn {
                MouseButton::Left => {
                    self.is_dragging = false;
                }
                _ => {}
            },
            _ => {}
        });

        event.map(|e, _| match e {
            CardEvent::DragChange(card) => {
                self.is_dragging = card.is_some();
                self.dragging = card.clone();
            }
        });
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub(crate) content: fn(&mut Context),
    pub(crate) drag: fn(&mut EventContext),
    pub(crate) spawn: fn() -> Arc<dyn AudioEffect>,
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

    pub fn spawn(&self) -> Arc<dyn AudioEffect> {
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
    spawn: || Arc::new(Overdrive::default()),
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
    spawn: || Arc::new(Fuzz::default()),
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
    spawn: || Arc::new(Fuzz::default()),
};

pub const REVERB_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Studio Reverb")
                .font_family(vec![FamilyOwned::Name("Saturday".to_owned())])
                .color(Color::rgb(232, 57, 57))
                .font_size(40.0);
        })
        .border_width(Pixels(2.0))
        .border_color(Color::rgb(232, 57, 57))
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(REVERB_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(StudioReverb::default()),
};
