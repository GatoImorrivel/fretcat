use fretcat_macros::Message;
use nih_plug::vizia::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    common::Filter,
    components::{Graph, LabeledKnobModifier, NamedKnob, Point},
    effects::AudioEffect,
    frame::Frame,
    EffectHandle,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HighPass {
    filter: [Filter; 2],
    min_freq_hz: f32,
    max_freq_hz: f32,
}

impl Default for HighPass {
    fn default() -> Self {
        let min_freq_hz = 20f32;
        let max_freq_hz = 20000f32;
        Self {
            min_freq_hz,
            max_freq_hz,
            filter: [Filter::new(
                crate::common::FilterMode::Highpass,
                44100.0,
                min_freq_hz,
                1.0,
            ); 2],
        }
    }
}

impl AudioEffect for HighPass {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        input_buffer.process_individual(|left, right| {
            *left = self.filter[0].tick(*left);
            *right = self.filter[1].tick(*right);
        });
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        HighPassView::new(_cx, EffectHandle::<Self>::from(_effect)).class("base-effect");
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens, Message)]
struct HighPassView {
    #[msg]
    cutoff: f32,
    #[msg]
    q: f32,
    graph_points: Vec<Point>,

    #[lens(ignore)]
    handle: EffectHandle<HighPass>,
}

impl HighPassView {
    pub fn new(cx: &mut Context, handle: EffectHandle<HighPass>) -> Handle<Self> {
        Self {
            cutoff: handle.filter[0].cutoff(),
            q: handle.filter[0].q(),
            handle: handle.clone(),
            graph_points: handle.filter[0].frequency_response()
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Graph::new(cx, Self::graph_points).class("filter-graph");
                NamedKnob::new(
                    cx,
                    "Cutoff",
                    Self::cutoff,
                    false,
                    handle.min_freq_hz..handle.max_freq_hz,
                )
                .on_changing(|ex, val| ex.emit(Message::Cutoff(val)))
                .class("filter-knob")
                .class("cutoff-knob")
                .height(Stretch(1.0))
                .width(Stretch(1.0));
                NamedKnob::new(cx, "Resonance", Self::q, false, 0.0..2.0)
                    .on_changing(|ex, val| ex.emit(Message::Q(val)))
                    .class("filter-knob")
                    .class("q-knob")
                    .height(Stretch(1.0))
                    .width(Stretch(1.0));
                Label::new(cx, "High Pass").class("effect-title");
            });
        })
    }
}

impl View for HighPassView {
    fn element(&self) -> Option<&'static str> {
        Some("high-pass")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Cutoff(val) => {
                self.cutoff = *val;
                self.handle
                    .filter
                    .iter_mut()
                    .for_each(|filter| filter.set_cutoff(*val));
                self.graph_points = self.handle.filter[0].frequency_response();
            }
            Message::Q(val) => {
                self.q = *val;
                self.handle
                    .filter
                    .iter_mut()
                    .for_each(|filter| filter.set_q(*val));
                self.graph_points = self.handle.filter[0].frequency_response();
            }
        })
    }
}
