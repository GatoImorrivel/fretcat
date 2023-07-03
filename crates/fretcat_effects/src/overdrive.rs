use nih_plug::{util::db_to_gain, nih_log};
use nih_plug_vizia::vizia::prelude::*;
use std::f32::consts::PI;

use crate::effect::{Effect, EffectHandle};
use fretcat_derive::Control;

#[derive(Debug, Clone, Copy, Default, Control)]
pub struct Overdrive {
    #[control]
    gain: f32,
    #[control]
    blend: f32,
    #[control]
    threshold: f32,
    #[control]
    volume: f32
}

impl Effect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let clean = _sample;
        let threshold = self.threshold * 100.0;
        let amplified = _sample * db_to_gain(self.gain * threshold);
        let distorted = (2.0 / PI) * f32::atan(amplified);

        let output_gain = db_to_gain(self.volume * 10.0);

        ((distorted * self.blend) + (clean * (1.0 - self.blend))) * output_gain
    }

    fn title(&self) -> String {
        "Drive".to_owned()
    }

    fn height(&self) -> f32 {
        200.0
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn view(&mut self, cx: &mut Context, handle: EffectHandle) {
        let o = handle.downcast_into::<Overdrive>();
        OverdriveControl {
            gain: o.gain,
            volume: o.volume,
            blend: o.blend,
            threshold: o.threshold,
            handle: handle
        }.build(cx, |cx| {
            cx.add_stylesheet(include_str!("../css/overdrive.css")).unwrap();
            HStack::new(cx, |cx| {
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
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::blend, false)
                        .on_changing(|cx, val| cx.emit(Message::Blend(val)));
                    Label::new(cx, "Blend");
                }).class("overdrive-knob-group");
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::volume, false)
                        .on_changing(|cx, val| cx.emit(Message::Volume(val)));
                    Label::new(cx, "Volume");
                }).class("overdrive-knob-group");
            })
            .class("overdrive")
            .height(Pixels(self.height()));
        });
    }
}

enum Message {
    Gain(f32), 
    Blend(f32), 
    Threshold(f32), 
    Volume(f32)
}

impl View for OverdriveControl {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
                self.downcast_mut_handle().gain = *val;
            },
            Message::Volume(val) => {
                self.volume = *val;
                self.downcast_mut_handle().volume = *val;
            },
            Message::Threshold(val) => {
                self.threshold = *val;
                self.downcast_mut_handle().threshold = *val;
            },
            Message::Blend(val) => {
                self.blend = *val;
                self.downcast_mut_handle().blend = *val;
            },
        });
    }
}