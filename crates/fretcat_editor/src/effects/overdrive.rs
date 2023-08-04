use std::{sync::Arc, marker::PhantomData};

use editor_derive::Control;
use fretcat_effects::{Overdrive, AtomicRefCell, Chain, Effect};
use nih_plug_vizia::vizia::prelude::*;

use super::EffectHandle;

#[derive(Debug, Lens, Control)]
struct OverdriveControl {
    pub gain: f32,
    pub threshold: f32,
}

impl Model for OverdriveControl {
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

impl EffectHandle<Overdrive> {
    pub fn new(
        cx: &mut Context,
        chain: Arc<AtomicRefCell<Chain>>,
        effect: &Effect,
        data: &Overdrive,
    ) {
        Self {
            chain: chain.clone(),
            effect: effect.clone(),
            p: PhantomData,
        }
        .build(cx, |cx| {
            OverdriveControl {
                gain: data.gain,
                threshold: data.threshold,
            }
            .build(cx);

            cx.add_stylesheet(include_str!("./overdrive.css")).unwrap();

            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::gain, false)
                        .on_changing(|cx, val| cx.emit(Message::Gain(val)));
                    Label::new(cx, "Gain");
                }).class("overdrive-knob-group");
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::threshold, false)
                        .on_changing(|cx, val| cx.emit(Message::Threshold(val)));
                    Label::new(cx, "Threshold");
                }).class("overdrive-knob-group");
            });
        });
    }
}

impl View for EffectHandle<Overdrive> {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                let ptr = self.chain.as_ptr();
                unsafe {
                    ptr.as_mut()
                        .unwrap()
                        .query_cast_mut::<Overdrive>(&self.effect)
                        .unwrap()
                        .gain = *val;
                }
            }
            Message::Threshold(val) => {
                let ptr = self.chain.as_ptr();

                unsafe {
                    ptr.as_mut()
                        .unwrap()
                        .query_cast_mut::<Overdrive>(&self.effect)
                        .unwrap()
                        .threshold = *val;
                }
            }
        });
    }
}