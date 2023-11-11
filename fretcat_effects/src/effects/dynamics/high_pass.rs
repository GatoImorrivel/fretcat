use nih_plug::vizia::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{EffectHandle, effects::AudioEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighPass {


}

impl Default for HighPass {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for HighPass {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        HighPassView::new(_cx, EffectHandle::<Self>::from(_effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens)]
struct HighPassView {

    #[lens(ignore)]
    handle: EffectHandle<HighPass>
}

impl HighPassView {
    pub fn new(cx: &mut Context, handle: EffectHandle<HighPass>) -> Handle<Self> {
        Self {
            handle: handle.clone()
        }.build(cx, |cx| {

        })
    }
}

impl View for HighPassView {
    fn element(&self) -> Option<&'static str> {
        Some("auto-wah")
    }
}