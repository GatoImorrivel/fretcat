use std::f32::consts::PI;

use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

use super::{Effect, common::tick_knob};

enum OverdriveMessage {
    GainChange(f32),
    BlendChange(f32),
    ThresholdChange(f32),
    VolumeChange(f32),
}

#[derive(Debug, Clone, Copy, Lens, Default)]
pub struct Overdrive {
    gain: f32,
    blend: f32,
    threshold: f32,
    volume: f32,
}

impl View for Overdrive {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            OverdriveMessage::GainChange(value) => {
                self.gain = *value;
            }
            OverdriveMessage::BlendChange(value) => {
                self.blend = *value;
            }
            OverdriveMessage::ThresholdChange(value) => {
                self.threshold = *value;
            }
            OverdriveMessage::VolumeChange(value) => {
                self.volume = *value;
            }
        });
    }
}

impl Effect for Overdrive {
    fn process(&self, _sample: f32) -> f32 {
        let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
        let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

        blend * self.volume
    }

    fn render(&self, cx: &mut Context) {
        self.build(cx, |cx| {
            cx.add_stylesheet(include_str!("./overdrive.css")).unwrap();
            HStack::new(cx, |cx: &mut Context| {
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::gain, false)
                        .on_changing(|cx, val| cx.emit(OverdriveMessage::GainChange(val)));
                    Label::new(cx, "Gain");
                });
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::blend, false)
                        .on_changing(|cx, val| cx.emit(OverdriveMessage::BlendChange(val)));
                    Label::new(cx, "Blend");
                });
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::threshold, false)
                        .on_changing(|cx, val| cx.emit(OverdriveMessage::ThresholdChange(val)));
                    Label::new(cx, "Threshold");
                });
                VStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::volume, false)
                        .on_changing(|cx, val| cx.emit(OverdriveMessage::VolumeChange(val)));
                    Label::new(cx, "Volume");
                });
            })
            .class("overdrive");
        });
    }
}
