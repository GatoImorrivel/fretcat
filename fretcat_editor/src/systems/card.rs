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
            Label::new(cx, "GAIN BOOSTER");
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
            Label::new(cx, "DRIVE");
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
            Label::new(cx, "FUZZ");
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
            Label::new(cx, "DISTORTION");
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
            Label::new(cx, "BIT CRUSHER");
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
            Label::new(cx, "STUDIO REVERB");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(REVERB_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(StudioReverb::default()),
};

pub const DELAY_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "DELAY");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(DELAY_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(MonoDelay::default()),
};

pub const TWIN_DELAY_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "TWIN DELAY");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(TWIN_DELAY_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(TwinDelay::default()),
};

pub const AUTO_WAH_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "AUTO WAH");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(AUTO_WAH_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(AutoWah::default()),
};

pub const LOW_PASS_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "LOW PASS");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(LOW_PASS_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(LowPass::default()),
};

pub const HIGH_PASS_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "HIGH PASS");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(HIGH_PASS_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(HighPass::default()),
};

pub const BAND_PASS_CARD: Card = Card {
    content: |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "BAND PASS");
        })
        .child_space(Stretch(1.0));
    },
    drag: |ex| {
        ex.emit(CardEvent::DragChange(Some(BAND_PASS_CARD)));
        ex.set_drop_data(ex.current());
    },
    spawn: || Arc::new(BandPass::default()),
};