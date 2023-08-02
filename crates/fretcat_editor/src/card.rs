

use fretcat_effects::{effect::{Effect, AudioEffect}, fuzz::Fuzz, overdrive::Overdrive, EffectKind};

use nih_plug_vizia::vizia::prelude::*;

#[derive(Lens)]
pub struct CardData {
    pub(crate) dragging: Option<Card>,
    pub(crate) cursor: (f32, f32),
    pub(crate) effect_kinds: Vec<EffectKind>,
    pub(crate) selected_kind: usize
}

pub enum CardEvent {
    DragChange(Option<Card>),
    KindChange(usize)
}

impl Model for CardData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseMove(x, y) => {
                self.cursor = (*x, *y);
            },
            WindowEvent::MouseUp(btn) => {
                match btn {
                    MouseButton::Left => {
                        self.dragging = None;
                    },
                    _ => {}
                }
            }
            _ => {}
        });

        event.map(|e, _| match e {
            CardEvent::DragChange(card) => {
                self.dragging = card.clone();
            },
            CardEvent::KindChange(index) => {
                self.selected_kind = *index;
            }
        });
    }
}

#[derive(Clone, Copy)]
pub struct Card {
    pub(crate) content: fn(&mut Context),
    pub(crate) drag: fn(&mut EventContext),
    pub(crate) spawn: fn() -> Box<dyn AudioEffect>
}

impl Card {
    pub fn render(&self, cx: &mut Context) {
        VStack::new(cx, self.content).on_drag(self.drag)
        .class("card-base");
    }

    pub fn content(&self, cx: &mut Context) {
        VStack::new(cx, self.content).class("card-base");
    }

    pub fn spawn(&self) -> Box<dyn AudioEffect> {
        (self.spawn)()
    }
}

impl Data for Card {
    fn same(&self, other: &Self) -> bool {
        let b1 = self.spawn();
        let b2 = other.spawn();

        let left = b1.as_ref() as *const _;
        let right = b2.as_ref() as *const _;
        left == right
    }
}

pub fn card_drag(cx: &mut Context) {
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

pub const OVERDRIVE_CARD: Card = Card {
    content: |cx| {
        Label::new(cx, "Overdrive");
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(OVERDRIVE_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || {
        Box::new(Overdrive::default())
    }
};

pub const FUZZ_CARD: Card = Card {
    content: |cx| {
        Label::new(cx, "Fuzz");
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(FUZZ_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || {
        Box::new(Fuzz::default())
    }
};