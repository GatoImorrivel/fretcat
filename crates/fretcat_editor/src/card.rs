use std::{sync::Arc, cell::Cell};

use fretcat_effects::{effect::Effect, overdrive::Overdrive, fuzz::Fuzz};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Lens)]
pub struct CardData {
    pub(crate) dragging: Arc<Cell<Option<Box<dyn Effect + Send + Sync>>>>
}

enum CardEvent {
    DragChange(Option<Box<dyn Effect + Send + Sync>>)
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

/*
fn card_base(cx: &mut Context, spawn: Box<dyn Effect + Send + Sync>, content: fn(&mut Context)) {
    cx.add_stylesheet(include_str!("../css/cards.css")).unwrap();
    VStack::new(cx, content)
    .class("card-base")
    .on_drag(move |ex| {
        ex.emit(CardEvent::DragChange(Some(spawn)));
    });
}*/

pub fn overdrive_card(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Overdrive");
    }).on_drag(|ex| {
        nih_log!("DRAGGING");
        let o = Box::new(Overdrive::default());
        ex.emit(CardEvent::DragChange(Some(o)));
        ex.set_drop_data(ex.current());
    })
    .class("card-base");
}

pub fn fuzz_card(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Overdrive");
    }).on_drag(|ex| {
        nih_log!("DRAGGING");
        let o = Box::new(Overdrive::default());
        ex.emit(CardEvent::DragChange(Some(o)));
        ex.set_drop_data(ex.current());
    })
    .class("card-base");
}