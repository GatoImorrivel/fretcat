use std::{marker::PhantomData, sync::Arc};

use editor_derive::Control;
use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, Effect, Overdrive};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Clone)]
pub struct EffectHandle<T: AudioEffect> {
    effect: Effect,
    chain: Arc<AtomicRefCell<Chain>>,
    p: PhantomData<T>,
}

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

            VStack::new(cx, |cx| {
                Knob::new(cx, 0.0, OverdriveControl::threshold, false)
                    .on_changing(|ex, val| ex.emit(Message::Threshold(val)));
                Knob::new(cx, 0.0, OverdriveControl::gain, false)
                    .on_changing(|ex, val| ex.emit(Message::Gain(val)));
            })
            .background_color(Color::red());
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
