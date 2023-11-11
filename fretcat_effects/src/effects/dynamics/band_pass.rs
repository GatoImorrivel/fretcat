use nih_plug::vizia::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{EffectHandle, effects::AudioEffect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandPass {


}

impl Default for BandPass {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for BandPass {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, _cx: &mut Context, _effect: std::sync::Arc<dyn AudioEffect>) {
        BandPassView::new(_cx, EffectHandle::<Self>::from(_effect));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens)]
struct BandPassView {

    #[lens(ignore)]
    handle: EffectHandle<BandPass>
}

impl BandPassView {
    pub fn new(cx: &mut Context, handle: EffectHandle<BandPass>) -> Handle<Self> {
        Self {
            handle: handle.clone()
        }.build(cx, |cx| {

        })
    }
}

impl View for BandPassView {
    fn element(&self) -> Option<&'static str> {
        Some("auto-wah")
    }
}