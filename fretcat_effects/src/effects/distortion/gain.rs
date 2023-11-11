use fretcat_macros::Message;
use nih_plug::{util::db_to_gain_fast, vizia::prelude::*};
use serde::{Serialize, Deserialize};

use crate::{components::{LabeledKnob, LabeledKnobModifier}, EffectHandle, effects::AudioEffect};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Gain {
    pub gain_in_db: f32,
}

impl Default for Gain {
    fn default() -> Self {
        Self { gain_in_db: 1.0 }
    }
}

impl AudioEffect for Gain {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
        input_buffer
            .0
            .iter_mut()
            .zip(input_buffer.1.iter_mut())
            .for_each(|(left, right)| {
                *left *= db_to_gain_fast(self.gain_in_db);
                *right *= db_to_gain_fast(self.gain_in_db);
            });
    }

    fn view(&self, cx: &mut Context, effect: std::sync::Arc<dyn AudioEffect>) {
        GainView::new(cx, EffectHandle::<Self>::from(effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens, Message)]
struct GainView {
    #[msg]
    gain: f32,

    #[lens(ignore)]
    handle: EffectHandle<Gain>,
}

impl GainView {
    pub fn new(cx: &mut Context, handle: EffectHandle<Gain>) -> Handle<Self> {
        Self {
            gain: handle.gain_in_db,
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            LabeledKnob::new(
                cx,
                Self::gain,
                false,
                -60.0..20.0,
            ).on_changing(|ex, val| ex.emit(Message::Gain(val)));
        })
    }
}

impl View for GainView {
    fn element(&self) -> Option<&'static str> {
        Some("gain-view")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
                self.handle.gain_in_db = *val;
            }
        });
    }
}
