use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinDelay {


}

impl Default for TwinDelay {
    fn default() -> Self {
        Self {  }
    }
}

impl AudioEffect for TwinDelay {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        TwinDelayView::new(cx, EffectHandle::<Self>::from(handle));
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