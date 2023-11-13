use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoWah {


}

impl Default for AutoWah {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for AutoWah {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        AutoWahView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens)]
struct AutoWahView {

    #[lens(ignore)]
    handle: EffectHandle<AutoWah>
}

impl AutoWahView {
    pub fn new(cx: &mut Context, handle: EffectHandle<AutoWah>) -> Handle<Self> {
        Self {
            handle: handle.clone()
        }.build(cx, |cx| {

        })
    }
}

impl View for AutoWahView {
    fn element(&self) -> Option<&'static str> {
        Some("auto-wah")
    }
}