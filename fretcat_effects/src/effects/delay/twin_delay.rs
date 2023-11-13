use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinDelay {
    wet_l: f32,
    wet_r: f32,
    delays: [Delay; 2],
}

impl Default for TwinDelay {
    fn default() -> Self {
        Self {
            wet_l: 0.5,
            wet_r: 0.5,
            delays: [Delay::default(), Delay::default()],
        }
    }
}

impl AudioEffect for TwinDelay {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        if transport.sample_rate != self.delays[0].sample_rate() {
            nih_plug::util::permit_alloc(|| {
                self.delays[0].set_sample_rate(transport.sample_rate);
                self.delays[1].set_sample_rate(transport.sample_rate);
            });
        }

        input_buffer.process_individual(|left, right| {
            nih_plug::util::permit_alloc(|| {
                *left = ((1.0 - self.wet_l) * *left) + (self.wet_l * self.delays[0].tick(*left));
                *right = ((1.0 - self.wet_r) * *right) + (self.wet_r * self.delays[1].tick(*right));
            });
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        TwinDelayView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
    }

    fn height(&self) -> f32 {
        200.0
    }
}

#[derive(Debug, Clone, Lens, Message)]
struct TwinDelayView {
    #[msg]
    time_l: f32,
    #[msg]
    feedback_l: f32,
    #[msg]
    wet_l: f32,

    #[msg]
    time_r: f32,
    #[msg]
    feedback_r: f32,
    #[msg]
    wet_r: f32,

    #[lens(ignore)]
    handle: EffectHandle<TwinDelay>,
}

impl TwinDelayView {
    pub fn new(cx: &mut Context, handle: EffectHandle<TwinDelay>) -> Handle<Self> {
        Self {
            wet_l: handle.wet_l * 100.0,
            time_l: handle.delays[0].delay_time_secs() * 1000.0,
            feedback_l: handle.delays[0].feedback() * 100.0,

            wet_r: handle.wet_r * 100.0,
            time_r: handle.delays[1].delay_time_secs() * 1000.0,
            feedback_r: handle.delays[1].feedback() * 100.0,

            handle: handle.clone(),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        NamedKnob::new(cx, "Time", Self::time_l, false, 20.0..1000.0)
                            .on_changing(|ex, val| ex.emit(Message::Time_l(val)));
                        Label::new(cx, "Left").class("side-indicator");
                        NamedKnob::new(cx, "Feedback", Self::feedback_l, false, 0.0..100.0)
                            .on_changing(|ex, val| ex.emit(Message::Feedback_l(val)));
                    })
                    .class("main-controls");
                    NamedKnob::new(cx, "Wet", Self::wet_l, false, 0.0..100.0)
                        .on_changing(|ex, val| ex.emit(Message::Wet_l(val)));
                })
                .class("knob-group");
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        NamedKnob::new(cx, "Time", Self::wet_r, false, 20.0..1000.0)
                            .on_changing(|ex, val| ex.emit(Message::Time_r(val)));
                        Label::new(cx, "Right").class("side-indicator");
                        NamedKnob::new(cx, "Feedback", Self::feedback_r, false, 0.0..100.0)
                            .on_changing(|ex, val| ex.emit(Message::Feedback_r(val)));
                    })
                    .class("main-controls");
                    NamedKnob::new(cx, "Wet", Self::wet_r, false, 0.0..100.0)
                        .on_changing(|ex, val| ex.emit(Message::Wet_r(val)));
                })
                .class("knob-group");
                Label::new(cx, "TWIN DELAY").class("effect-title");
            });
        })
    }
}

impl View for TwinDelayView {
    fn element(&self) -> Option<&'static str> {
        Some("twin-delay")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Feedback_l(val) => {
                self.feedback_l = *val;
                self.handle.delays[0].set_feedback(*val / 100.0);
            }

            Message::Feedback_r(val) => {
                self.feedback_r = *val;
                self.handle.delays[1].set_feedback(*val / 100.0);
            }

            Message::Time_l(val) => {
                self.time_l = *val;
                self.handle.delays[0].set_delay_time_secs(*val / 1000.0);
            }

            Message::Time_r(val) => {
                self.time_r = *val;
                self.handle.delays[1].set_delay_time_secs(*val / 1000.0);
            }

            Message::Wet_l(val) => {
                self.wet_l = *val;
                self.wet_l = *val / 100.0;
            }

            Message::Wet_r(val) => {
                self.wet_r = *val;
                self.wet_r = *val / 100.0;
            }
        });
    }
}
