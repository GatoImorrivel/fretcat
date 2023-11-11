use std::{
    fmt::Debug,
    ops::Range,
};

use nih_plug::vizia::prelude::*;

use crate::common::normalize_value;

#[derive(Lens)]
pub struct LabeledKnob {
    knob_value: f32,
    pub real_value: f32,
    pub range: Range<f32>,

    #[lens(ignore)]
    on_changing: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

impl Debug for LabeledKnob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LabeledKnob")
            .field("real_value", &self.real_value)
            .field("range", &self.range)
            .finish()
    }
}


enum LabeledKnobEvent {
    Value(f32),
}

impl LabeledKnob {
    pub fn new<L: Lens<Target = f32>>(cx: &mut Context, value: L, centered: bool, range: Range<f32>) -> Handle<Self> {
        let normalized = normalize_value(value.get(cx), &range);
        Self {
            real_value: value.get(cx),
            knob_value: normalized,
            range: range,
            on_changing: None,
        }
        .build(cx, |cx| {
            ZStack::new(cx, |cx| {
                Knob::new(cx, normalized, Self::knob_value, centered)
                    .on_changing(|ex, val| ex.emit(LabeledKnobEvent::Value(val)));
                Label::new(cx, Self::real_value.map(|val| format!("{:.0}", val))).class("knob-value");
            }).child_space(Stretch(1.0));
        })
    }
}

pub trait LabeledKnobModifier {
    fn on_changing<F: Fn(&mut EventContext, f32) + 'static>(self, f: F) -> Self;
}

impl<'a> LabeledKnobModifier for Handle<'a, LabeledKnob> {
    fn on_changing<F: Fn(&mut EventContext, f32) + 'static>(self, f: F) -> Self {
        self.modify(|view: &mut LabeledKnob| view.on_changing = Some(Box::new(f)))
    }
}

impl View for LabeledKnob {
    fn element(&self) -> Option<&'static str> {
        Some("labeled-knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            LabeledKnobEvent::Value(val) => {
                self.knob_value = *val;
                self.real_value = self.range.start + (self.range.end - self.range.start) * *val;
                if let Some(f) = &self.on_changing {
                    (f)(cx, self.real_value);
                }
            }
        });
    }
}
