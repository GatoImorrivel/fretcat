use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Clone, Copy, Lens)]
pub struct ChannelSlider {
    pub value: f32,
    pub dragging: bool,
    pub height: f32,
}

enum ChannelSliderMessage {
    ValueChanged(f32),
    Dragging(bool),
}

impl View for ChannelSlider {
    fn element(&self) -> Option<&'static str> {
        Some("channel-slider")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            ChannelSliderMessage::ValueChanged(val) => {
                self.value *= val;
            }
            ChannelSliderMessage::Dragging(val) => {
                self.dragging = *val;
            }
        });

        event.map(|e, _| match e {
            WindowEvent::MouseMove(_, y) => {
                if !self.dragging {
                    return;
                }

                nih_log!("DRAGGING: {}", *y);
                if *y < cx.bounds().bottom() {
                    self.value = 0.0;
                }

                if *y > cx.bounds().top() {
                    self.value = 1.0;
                }
            }
            WindowEvent::MouseUp(_) => {
                self.dragging = false;
            }
            _ => {}
        });
    }
}

impl ChannelSlider {
    pub fn new(cx: &mut Context, width: f32, height: f32) {
        Self {
            value: 0.0,
            dragging: false,
            height,
        }
        .build(cx, |cx| {
            cx.add_stylesheet(include_str!("../../css/channel-slider.css"))
                .unwrap();
            Element::new(cx)
                .height(Pixels(height))
                .width(Pixels(width))
                .class("background");
            Binding::new(cx, ChannelSlider::value, move |cx, bind| {
                let val = bind.get(cx);
                let pos = val * height;
                Element::new(cx)
                    .on_mouse_down(|cx, b| {
                        if b == MouseButton::Left {
                            cx.emit(ChannelSliderMessage::Dragging(true));
                        }
                    })
                    .bottom(Percentage(pos))
                    .height(Pixels(width))
                    .class("drag-ball");
            });
        });
    }
}
