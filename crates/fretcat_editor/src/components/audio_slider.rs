use std::marker::PhantomData;

use nih_plug::{
    prelude::Vst3Plugin,
    util::{gain_to_db, gain_to_db_fast, gain_to_db_fast_epsilon},
    vizia::prelude::*,
};

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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AudioSliderMessage::Gain(val) => {
                self.gain = *val;
            }
        });
    }
}

impl<L: Lens<Target = (f32, f32)>> AudioSlider<L> {
    pub fn new(cx: &mut Context, height: f32, lens: L) {
        Self {
            gain: 0.0,
            _p: PhantomData,
        }
        .build(cx, |cx| {
            ZStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Binding::new(cx, lens, |cx, bind| {
                        let channels = bind.get(cx);
                        ZStack::new(cx, |cx| {
                            Element::new(cx)
                                .class("audio-slider-bg")
                                .width(Stretch(1.0))
                                .height(Stretch(1.0));
                            Element::new(cx)
                                .background_color(Color::black())
                                .width(Stretch(1.0))
                                .height(Percentage(
                                    100.0 - normalize(channels.0, -100.0, 6.0, 0.0, 100.0),
                                ));
                        });
                        ZStack::new(cx, |cx| {
                            Element::new(cx)
                                .class("audio-slider-bg")
                                .width(Stretch(1.0))
                                .height(Stretch(1.0));
                            Element::new(cx)
                                .background_color(Color::black())
                                .width(Stretch(1.0))
                                .height(Percentage(
                                    100.0 - normalize(channels.1, -100.0, 6.0, 0.0, 100.0),
                                ));
                        });
                    });
                })
                .height(Pixels(height))
                .width(Stretch(1.0))
                .col_between(Percentage(5.0));
                Slider::new(cx, Self::gain)
                    .height(Pixels(height))
                    .width(Stretch(1.0))
                    .on_changing(|ex, val| ex.emit(AudioSliderMessage::Gain(val)));
            });
        });
    }
}

#[inline]
fn normalize(value: f32, min_input: f32, max_input: f32, min_output: f32, max_output: f32) -> f32 {
    let clamped_value = value.max(min_input).min(max_input);

    let input_range = max_input - min_input;
    let output_range = max_output - min_output;
    let normalized = (clamped_value - min_input) * output_range / input_range + min_output;

    normalized
}
