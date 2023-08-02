use nih_plug_vizia::vizia::prelude::*;
use std::f32::consts::PI;

use crate::effect::{Effect, AudioEffect};
use crate::chain::ChainHandle;
use crate::overdrive::OverdriveControl;
use fretcat_derive::Control;

#[derive(Debug, Clone, Copy, Default, Control)]
pub struct Fuzz {
    #[control]
    gain: f32,
    blend: f32,
    threshold: f32,
    #[control]
    volume: f32
}

impl AudioEffect for Fuzz {
    fn process(&self, _sample: f32) -> f32 {
        let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
        let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

        blend * self.volume
    }

    fn title(&self) -> String {
        "Fuzz".to_owned()
    }

    fn height(&self) -> f32 {
        400.0
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn view(&self, cx: &mut Context, handle: &Effect) {
        create_control(cx, handle, |cx| {
            cx.add_stylesheet(include_str!("../css/overdrive.css")).unwrap();
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, FuzzControl::gain, false)
                        .on_changing(|cx, val| cx.emit(Message::Gain(val)));
                }).class("overdrive-knob-group");
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, FuzzControl::volume, false)
                        .on_changing(|cx, val| cx.emit(Message::Volume(val)));
                }).class("overdrive-knob-group");
            })
            .class("overdrive")
            .height(Pixels(self.height()));
        });
    }
}