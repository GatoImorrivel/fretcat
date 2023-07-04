use std::sync::Arc;

use fretcat_effects::{effect::Effect, overdrive::Overdrive, fuzz::Fuzz};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Clone, Lens)]
pub struct CardData {
    pub(crate) dragging: Option<Arc<dyn Effect + Send + Sync>>
}

enum CardEvent {
    DragChange(Option<Arc<dyn Effect + Send + Sync>>)
}

impl Model for CardData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            CardEvent::DragChange(t) => {
                self.dragging = t.clone();
            }
        })
    }
}

fn card_base(cx: &mut Context, spawn: Arc<dyn Effect + Send + Sync>, content: fn(&mut Context)) {
    cx.add_stylesheet(include_str!("../css/cards.css")).unwrap();
    VStack::new(cx, content)
    .class("card-base")
    .on_drag(move |ex| {
        nih_log!("{:#?}", spawn);
        ex.emit(CardEvent::DragChange(Some(spawn.clone())));
        ex.set_drop_data(ex.current());
    });
}

pub fn overdrive_card(cx: &mut Context) {
    let o = Overdrive::default();
    card_base(cx, Arc::new(o), |cx| {
        Label::new(cx, "Overdrive");
    });
}

pub fn fuzz_card(cx: &mut Context) {
    let f = Fuzz::default();
    card_base(cx, Arc::new(f), |cx| {
        Label::new(cx, "Fuzz");
    });
}