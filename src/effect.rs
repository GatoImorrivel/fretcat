use core::fmt;
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect: fmt::Debug {
    fn process(&self, _sample: f32) -> f32;
    fn ui(&self, cx: &mut Context);
}

pub mod overdrive {
    use std::f32::consts::PI;

    use nih_plug_vizia::vizia::{
        prelude::*,
    };

    use crate::editor::common::tick_knob;

    use super::Effect;

    enum OverdriveMessage {
        GainChange(f32),
    }

    #[derive(Debug, Clone, Copy, Lens)]
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
            });
        }
    }

    impl Default for Overdrive {
        fn default() -> Self {
            Self {
                gain: 0.0,
                blend: 0.1,
                threshold: 50.0,
                volume: 1.0,
            }
        }
    }

    impl Effect for Overdrive {
        fn process(&self, _sample: f32) -> f32 {
            let dirty = (2.0 / PI) * f32::atan(_sample * self.gain * self.threshold);
            let blend = ((dirty * self.blend) + (_sample * (1.0 / self.blend))) / 2.0;

            blend * self.volume
        }

        fn ui(&self, cx: &mut Context) {
            self.build(cx, |cx| {
                HStack::new(cx, |cx: &mut Context| {
                    tick_knob(cx, Self::gain).on_changing(move |cx, val| {
                        cx.emit(OverdriveMessage::GainChange(val));
                    });
                });
            });
        }
    }
}
