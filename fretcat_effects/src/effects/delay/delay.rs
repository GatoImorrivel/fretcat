use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonoDelay {
    wet: f32,
    delays: [Delay; 2]
}

impl MonoDelay {
    pub fn set_feedback(&mut self, feedback: f32) {
        self.delays.iter_mut().for_each(|delay| {
            delay.set_feedback(feedback);
        });
    }

    pub fn set_time(&mut self, time_in_seconds: f32) {
        self.delays.iter_mut().for_each(|delay| {
            delay.set_delay_time_secs(time_in_seconds);
        });
    }

    pub fn set_wet(&mut self, wet: f32) {
        self.wet = wet.clamp(0.0, 1.0);
    }
}

impl Default for MonoDelay {
    fn default() -> Self {
        Self {  
            wet: 0.5,
            delays: [Delay::default(), Delay::default()]
        }
    }
}

impl AudioEffect for MonoDelay {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        if transport.sample_rate != self.delays[0].sample_rate() {
            nih_plug::util::permit_alloc(|| {
                self.delays[0].set_sample_rate(transport.sample_rate);
                self.delays[1].set_sample_rate(transport.sample_rate);
            });
        }

        input_buffer.process_individual(|left, right| {
            nih_plug::util::permit_alloc(|| {
                *left = ((1.0 - self.wet) * *left) + (self.wet * self.delays[0].tick(*left));
                *right = ((1.0 - self.wet) * *right) + (self.wet * self.delays[1].tick(*right));
            });
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        DelayView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens, Message)]
struct DelayView {
    #[msg]
    time: f32,
    #[msg]
    feedback: f32,
    #[msg]
    wet: f32,

    #[lens(ignore)]
    handle: EffectHandle<MonoDelay>
}

impl DelayView {
    pub fn new(cx: &mut Context, handle: EffectHandle<MonoDelay>) -> Handle<Self> {
        Self {
            wet: handle.wet * 100.0,
            time: handle.delays[0].delay_time_secs() * 1000.0,
            feedback: handle.delays[0].feedback() * 100.0,
            handle: handle.clone()
        }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Time", Self::time, false, 20.0..1000.0)
                    .on_changing(|ex, val| ex.emit(Message::Time(val)));
                NamedKnob::new(cx, "Feedback", Self::feedback, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Feedback(val)));
                NamedKnob::new(cx, "Wet", Self::wet, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Wet(val)));
                Label::new(cx, "DELAY").class("effect-title");
            });
        })
    }
}

impl View for DelayView {
    fn element(&self) -> Option<&'static str> {
        Some("delay")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Feedback(val) => {
                self.feedback = *val;
                self.handle.set_feedback(*val / 100.0);
            }
            Message::Time(val) => {
                self.time = *val;
                self.handle.set_time(*val / 1000.0);
            }
            Message::Wet(val) => {
                self.wet = *val;
                self.handle.set_wet(*val / 100.0);
            }
        });
    }
}