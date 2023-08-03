use nih_plug::util::db_to_gain;
use nih_plug_vizia::vizia::prelude::*;
use std::f32::consts::PI;

use crate::{
    chain::ChainHandle,
    effect::{AudioEffect, Effect},
};
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
    volume: f32,
}

impl AudioEffect for Overdrive {
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

    fn view(&self, cx: &mut Context, handle: &Effect) {
        create_control(cx, handle, |cx| {
            cx.add_stylesheet(include_str!("../css/overdrive.css"))
                .unwrap();
            HStack::new(cx, |cx| {
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
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::blend, false)
                        .on_changing(|cx, val| cx.emit(Message::Blend(val)))
                        .needs_redraw();
                    Label::new(cx, "Blend");
                })
                .class("overdrive-knob-group");
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, OverdriveControl::volume, false)
                        .on_changing(|cx, val| cx.emit(Message::Volume(val)))
                        .needs_redraw();
                    Label::new(cx, "Volume");
                })
                .class("overdrive-knob-group");
            })
            .class("overdrive")
            .height(Pixels(self.height()));
        });
    }
}
