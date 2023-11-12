use nih_plug::vizia::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{EffectHandle, effects::AudioEffect, frame::Frame};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delay {


}

impl Default for Delay {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for Delay {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        DelayView::new(_cx, EffectHandle::<Self>::from(_effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens)]
struct DelayView {

    #[lens(ignore)]
    handle: EffectHandle<Delay>
}

impl DelayView {
    pub fn new(cx: &mut Context, handle: EffectHandle<Delay>) -> Handle<Self> {
        Self {
            handle: handle.clone()
        }.build(cx, |cx| {

        })
    }
}

impl View for DelayView {
    fn element(&self) -> Option<&'static str> {
        Some("delay")
    }
}