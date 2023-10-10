use crate::{common::Freeverb, NUM_CHANNELS};
use fretcat_common::vizia::prelude::*;
use fretcat_macros::{getter, Message};

use super::AudioEffect;

use crate::ChainData;

#[derive(Debug, Clone, Message, serde::Serialize, serde::Deserialize)]
pub struct StudioReverb {
    #[msg]
    pub wet: f32,
    #[msg]
    pub size: f32,

    reverb: Freeverb,
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

    fn view(&self, cx: &mut Context, effect: usize) {
        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(size), false)
                    .on_changing(|cx, val| cx.emit(Message::Size(val)))
                    .class("size-knob");
                Label::new(cx, "Room Size");
            })
            .class("studio-reverb-knob-group");
            HStack::new(cx, |cx| {
                Knob::new(cx, 1.0, getter!(wet), false)
                    .on_changing(|cx, val| cx.emit(Message::Wet(val)))
                    .class("wet-knob");
                Label::new(cx, "Wet");
            })
            .class("studio-reverb-knob-group");
        })
        .class("studio-reverb");
    }

    fn update(&self, event: &mut Event, effect: usize, chain: &mut crate::Chain) -> Option<()> {
        let data = chain.query_cast_mut::<Self>(effect)?;

        event.map(|e, _| match e {
            Message::Size(val) => {
                data.size = *val;
                data.reverb.set_room_size(*val);
            }
            Message::Wet(val) => {
                data.wet = *val;
                data.reverb.set_wet(*val);
            }
        });

        Some(())
    }

    fn height(&self) -> f32 {
        100.0
    }
}
