use std::{marker::PhantomData, sync::Arc};

use editor_derive::Control;
use fretcat_effects::{AtomicRefCell, Chain, Effect, Overdrive};
use nih_plug_vizia::vizia::prelude::*;

use super::{Control, EffectHandle};

use nih_plug::nih_log;

#[derive(Debug, Lens, Control)]
pub struct OverdriveControl {
    pub gain: f32,
    pub threshold: f32,
}

impl View for OverdriveControl {
    fn element(&self) -> Option<&'static str> {
        Some("overdrivecontrol")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
            }
            Message::Threshold(val) => {
                self.threshold = *val;
            }
        });
    }
}

impl Control<Overdrive> for OverdriveControl {
    type Message = Message;
    fn view(cx: &mut Context) {
        cx.add_stylesheet(include_str!("./overdrive.css")).unwrap();

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, OverdriveControl::gain, false)
                    .on_changing(|cx, val| cx.emit(Message::Gain(val)));
                Label::new(cx, "Gain");
            })
            .class("overdrive-knob-group");
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, OverdriveControl::threshold, false)
                    .on_changing(|cx, val| cx.emit(Message::Threshold(val)));
                Label::new(cx, "Threshold");
            })
            .class("overdrive-knob-group");
        });
    }

    fn update(event: &Self::Message, data: &mut Overdrive) {
        match event {
            Message::Gain(val) => {
                data.gain = *val;
            }
            Message::Threshold(val) => {
                data.threshold = *val;
            }
        }
    }

    fn init(data: &Overdrive) -> Self {
        Self {
            gain: data.gain,
            threshold: data.threshold,
        }
    }
}
