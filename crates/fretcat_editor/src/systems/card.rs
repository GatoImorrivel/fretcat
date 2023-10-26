use std::sync::Arc;

use fretcat_effects::effects::*;
use nih_plug::vizia::prelude::*;

use super::CardEvent;

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
