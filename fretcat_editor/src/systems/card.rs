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

pub const GAIN_BOOSTER_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Gain Booster");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(GAIN_BOOSTER_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(Gain::default()),
};

pub const OVERDRIVE_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Drive");
        })
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
            Label::new(cx, "Fuzz");
        })
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
            Label::new(cx, "Distortion");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(DISTORTION_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(Fuzz::default()),
};

pub const BIT_CRUSHER_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Bit Crusher");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(BIT_CRUSHER_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(BitCrusher::default()),
};

pub const REVERB_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Studio Reverb");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(REVERB_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(StudioReverb::default()),
};