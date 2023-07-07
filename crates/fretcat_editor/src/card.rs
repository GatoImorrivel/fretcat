use std::{cell::Cell, sync::Arc};

use fretcat_effects::{effect::Effect, fuzz::Fuzz, overdrive::Overdrive};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Lens)]
pub struct CardData {
    pub(crate) dragging: Arc<Cell<Option<Box<dyn Effect + Send + Sync>>>>,
}

enum CardEvent {
    DragChange(Option<Box<dyn Effect + Send + Sync>>),
}

impl Model for CardData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        let e = event.take();
        if let Some(e) = e {
            match e {
                CardEvent::DragChange(item) => {
                    self.dragging = Arc::new(Cell::new(item));
                }
            }
        }
    }
}

pub struct Card {
    pub(crate) content: fn(&mut Context),
    pub(crate) drag: fn(&mut EventContext),
}

impl Card {
    pub fn render(&self, cx: &mut Context) {
        VStack::new(cx, self.content).on_drag(self.drag)
        .class("card-base");
    }

    pub fn content(&self, cx: &mut Context) {
        VStack::new(cx, self.content).class("card-base");
    }
}

pub const OVERDRIVE_CARD: Card = Card {
    content: |cx| {
        Label::new(cx, "Overdrive");
    },
    drag: |ex| {
        let o = Box::new(Overdrive::default());
        ex.emit(CardEvent::DragChange(Some(o)));
        ex.set_drop_data(ex.current());
    }
};

pub const FUZZ_CARD: Card = Card {
    content: |cx| {
        Label::new(cx, "Fuzz");
    },
    drag: |ex| {
        let o = Box::new(Fuzz::default());
        ex.emit(CardEvent::DragChange(Some(o)));
        ex.set_drop_data(ex.current());
    }
};