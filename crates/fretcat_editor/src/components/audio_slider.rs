use std::marker::PhantomData;

use nih_plug::{vizia::prelude::*, util::{gain_to_db_fast, gain_to_db_fast_epsilon, gain_to_db}, prelude::Vst3Plugin};

#[derive(Debug, Clone, Copy, Lens)]
pub struct AudioSlider<L: Lens<Target = (f32, f32)>> {
    pub value: f32,
    pub dragging: bool,
    _p: PhantomData<L>
}

enum AudioSliderMessage {
    ValueChanged(f32),
    Dragging(bool),
}

impl<L: Lens<Target = (f32, f32)>> View for AudioSlider<L> {
    fn element(&self) -> Option<&'static str> {
        Some("channel-slider")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AudioSliderMessage::ValueChanged(val) => {
                self.value *= val;
            }
            AudioSliderMessage::Dragging(val) => {
                self.dragging = *val;
            }
        });

        event.map(|e, _| match e {
            WindowEvent::MouseMove(_, y) => {
            }
            WindowEvent::MouseUp(_) => {
                self.dragging = false;
            }
            _ => {}
        });
    }
}

impl<L: Lens<Target = (f32, f32)>> AudioSlider<L> {
    pub fn new(cx: &mut Context, height: f32, lens: L) {
        Self {
            value: 0.0,
            dragging: false,
            _p: PhantomData
        }
        .build(cx, |cx| {
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
                            .height(Percentage(100.0 - normalize(channels.0, -100.0, 6.0, 0.0, 100.0)));
                    });
                    ZStack::new(cx, |cx| {
                        Element::new(cx)
                            .class("audio-slider-bg")
                            .width(Stretch(1.0))
                            .height(Stretch(1.0));
                        Element::new(cx)
                            .background_color(Color::black())
                            .width(Stretch(1.0))
                            .height(Percentage(100.0 - normalize(channels.1, -100.0, 6.0, 0.0, 100.0)));
                        });
                    });
                })
            .height(Pixels(height))
            .width(Stretch(1.0))
            .col_between(Percentage(5.0));
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