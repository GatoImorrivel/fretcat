use nih_plug::vizia::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{EffectHandle, effects::AudioEffect};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinDelay {


}

impl Default for TwinDelay {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for TwinDelay {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        TwinDelayView::new(_cx, EffectHandle::<Self>::from(_effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens)]
struct TwinDelayView {

    #[lens(ignore)]
    handle: EffectHandle<TwinDelay>
}

impl TwinDelayView {
    pub fn new(cx: &mut Context, handle: EffectHandle<TwinDelay>) -> Handle<Self> {
        Self {
            handle: handle.clone()
        }.build(cx, |cx| {

        })
    }
}

impl View for TwinDelayView {
    fn element(&self) -> Option<&'static str> {
        Some("twin-delay")
    }
}