use nih_plug::vizia::{prelude::*, image::math::Rect};

#[derive(Debug, Clone, Copy, Lens)]
pub struct AudioSlider {
    pub value: f32,
    pub dragging: bool,
}

enum AudioSliderMessage {
    ValueChanged(f32),
    Dragging(bool),
}

impl View for AudioSlider {
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

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
    }
}

impl AudioSlider {
    pub fn new<L: Lens<Target = (f32, f32)>>(cx: &mut Context, width: f32, height: f32, lens: L) {
        Self {
            value: 0.0,
            dragging: false,
        }
        .build(cx, |cx| {
            let channel_amplitudes = lens.get(cx);
            let h1 = channel_amplitudes.0 / height;
            let h2 = channel_amplitudes.1 / height;
            HStack::new(cx, |cx| {
                Element::new(cx)
                    .height(Stretch(1.0))
                    .background_color(Color::red())
                    .height(Pixels(h1 * 100.0))
                    .width(Stretch(1.0));
                Element::new(cx)
                    .height(Stretch(1.0))
                    .background_color(Color::red())
                    .height(Pixels(h2 * 100.0))
                    .width(Stretch(1.0));
            }).col_between(Percentage(10.0));
        });
    }
}
