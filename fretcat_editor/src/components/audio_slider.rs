use std::marker::PhantomData;

use nih_plug::vizia::prelude::*;

use crate::common::{normalize, normalize_db};

#[derive(Debug, Clone, Copy, Lens)]
pub struct AudioSlider<L: Lens<Target = (f32, f32)>> {
    pub gain: f32,
    _p: PhantomData<L>,
}

enum AudioSliderMessage {
    Gain(f32),
}

impl<L: Lens<Target = (f32, f32)>> View for AudioSlider<L> {
    fn element(&self) -> Option<&'static str> {
        Some("audio-slider")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AudioSliderMessage::Gain(val) => {
                self.gain = *val;
            }
        });
    }
}

impl<L: Lens<Target = (f32, f32)>> AudioSlider<L> {
    pub fn new(cx: &mut Context, height: f32, lens: L, on_changing: fn(&mut EventContext, f32)) {
        Self {
            gain: 0.0,
            _p: PhantomData,
        }
        .build(cx, |cx| {
            Label::new(cx, Self::gain.map(|gain| format!("{:.1}db", gain)))
                .font_family(vec![FamilyOwned::Name("Sparkler-Demo".to_owned())])
                .color(Color::white())
                .font_size(12.0)
                .text_align(TextAlign::Center);
            ZStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    ZStack::new(cx, |cx| {
                        Element::new(cx)
                            .class("audio-slider-bg")
                            .width(Stretch(1.0))
                            .height(Stretch(1.0));
                        Element::new(cx)
                            .class("audio-slider-mask")
                            .width(Stretch(1.0))
                            .height(lens.map(|channels| {
                                Percentage(100.0 - normalize_db(channels.0, -100.0, 12.0, 0.0, 100.0))
                            }));
                    });
                    ZStack::new(cx, |cx| {
                        Element::new(cx)
                            .class("audio-slider-bg")
                            .width(Stretch(1.0))
                            .height(Stretch(1.0));
                        Element::new(cx)
                            .class("audio-slider-mask")
                            .width(Stretch(1.0))
                            .height(lens.map(|channels| {
                                Percentage(100.0 - normalize_db(channels.1, -100.0, 12.0, 0.0, 100.0))
                            }));
                    });
                })
                .height(Pixels(height))
                .width(Stretch(1.0))
                .col_between(Pixels(5.0));
                Slider::new(cx, Self::gain)
                    .range(-60.0..12.0)
                    .height(Pixels(height))
                    .width(Stretch(1.0))
                    .on_changing(move |ex, val| {
                        (on_changing)(ex, val);
                        ex.emit(AudioSliderMessage::Gain(val));
                    });
            });
        });
    }
}
