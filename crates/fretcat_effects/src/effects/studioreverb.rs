use std::sync::Arc;

use crate::{arc_to_mut, common::Freeverb};
use fretcat_macros::{Message};
use nih_plug::vizia::prelude::*;

use super::AudioEffect;



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudioReverb {
    pub wet: f32,
    pub size: f32,
    reverb: Freeverb,
}

impl PartialEq for StudioReverb {
    fn eq(&self, other: &Self) -> bool {
        self.wet == other.wet &&
        self.size == other.size
    }
}

impl Default for StudioReverb {
    fn default() -> Self {
        Self {
            wet: 0.5,
            size: 0.5,
            reverb: Freeverb::new(44100),
        }
    }
}

impl AudioEffect for StudioReverb {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32])) {
        input_buffer
            .0
            .iter_mut()
            .zip(input_buffer.1.iter_mut())
            .for_each(|(left, right)| {
                (*left, *right) = self.reverb.tick((*left, *right));
            });
    }

    fn view(&self, cx: &mut Context, effect: Arc<dyn AudioEffect>) {
        StudioReverbView::new(cx, effect.into_any_arc().downcast::<Self>().unwrap());
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Data, Lens, Message)]
pub struct StudioReverbView {
    #[msg]
    pub wet: f32,
    #[msg]
    pub size: f32,

    #[lens(ignore)]
    #[data(ignore)]
    effect: Arc<StudioReverb>,
}

impl StudioReverbView {
    pub fn new(cx: &mut Context, effect: Arc<StudioReverb>) -> Handle<Self> {
        Self {
            size: effect.size,
            wet: effect.wet,
            effect,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::size, false)
                        .on_changing(|cx, val| cx.emit(Message::Size(val)))
                        .class("size-knob");
                    Label::new(cx, "Room Size");
                })
                .class("studio-reverb-knob-group");
                HStack::new(cx, |cx| {
                    Knob::new(cx, 1.0, Self::wet, false)
                        .on_changing(|cx, val| cx.emit(Message::Wet(val)))
                        .class("wet-knob");
                    Label::new(cx, "Wet");
                })
                .class("studio-reverb-knob-group");
            })
            .class("studio-reverb");
        })
    }
}

impl View for StudioReverbView {
    fn element(&self) -> Option<&'static str> {
        Some("studio-reverb")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| {
            let effect = unsafe { arc_to_mut(&self.effect) };
            match e {
                Message::Size(val) => {
                    self.size = *val;
                    effect.reverb.set_room_size(*val);
                }
                Message::Wet(val) => {
                    self.wet = *val;
                    effect.reverb.set_wet(*val);
                }
            }
        });
    }
}
