use std::ops::Range;

use nih_plug::vizia::prelude::*;

use super::{LabeledKnob, LabeledKnobModifier};


#[derive(Lens)]
pub struct NamedKnob {
    #[lens(ignore)]
    on_changing: Option<Box<dyn Fn(&mut EventContext, f32)>>,
}

enum NamedKnobEvent {
    ChangeValue(f32),
}

impl NamedKnob {
    pub fn new<U, L>(
        cx: &mut Context,
        name: impl Res<U> + Clone,
        value: L,
        centered: bool,
        range: Range<f32>,
    ) -> Handle<Self>
    where
        U: ToString,
        L: Lens<Target = f32>,
    {
        Self { on_changing: None }.build(cx, |cx| {
            ZStack::new(cx, |cx| {
                LabeledKnob::new(cx, value, centered, range)
                    .on_changing(|ex, val| ex.emit(NamedKnobEvent::ChangeValue(val)));
                Label::new(cx, name).class("knob-name");
            });
        })
    }
}

impl<'a> LabeledKnobModifier for Handle<'a, NamedKnob> {
    fn on_changing<F: Fn(&mut EventContext, f32) + 'static>(self, f: F) -> Self {
        self.modify(|view: &mut NamedKnob| view.on_changing = Some(Box::new(f)))
    }
}

impl View for NamedKnob {
    fn element(&self) -> Option<&'static str> {
        Some("named-knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            NamedKnobEvent::ChangeValue(val) => {
                if let Some(f) = &self.on_changing {
                    (f)(cx, *val);
                }
            }
        });
    }
}
