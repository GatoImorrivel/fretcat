use fretcat_macros::Message;
use nih_plug::vizia::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{EffectHandle, effects::AudioEffect, frame::Frame, common::{Delay, normalize_value}, components::{NamedKnob, LabeledKnobModifier}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonoDelay {
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
}

impl Default for MonoDelay {
    fn default() -> Self {
        Self {  
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
                *left = self.delays[0].tick(*left);
                *right = self.delays[1].tick(*right);
            });
        });
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        DelayView::new(_cx, EffectHandle::<Self>::from(_effect));
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

    #[lens(ignore)]
    handle: EffectHandle<MonoDelay>
}

impl DelayView {
    pub fn new(cx: &mut Context, handle: EffectHandle<MonoDelay>) -> Handle<Self> {
        Self {
            time: handle.delays[0].delay_time_secs() * 1000.0,
            feedback: handle.delays[0].feedback() * 100.0,
            handle: handle.clone()
        }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Time", Self::time, false, 20.0..1000.0)
                    .on_changing(|ex, val| ex.emit(Message::Time(val)));
                NamedKnob::new(cx, "Feedback", Self::feedback, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Feedback(val)));
                Label::new(cx, "Delay").class("effect-title");
            });
        })
    }
}

impl View for DelayView {
    fn element(&self) -> Option<&'static str> {
        Some("delay")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Feedback(val) => {
                self.feedback = *val;
                self.handle.set_feedback(*val / 100.0);
            }
            Message::Time(val) => {
                self.time = *val;
                self.handle.set_time(*val / 1000.0);
            }
        });
    }
}