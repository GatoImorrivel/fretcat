use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitCrusher {
    bit_rate: f32,
}

impl Default for BitCrusher {
    fn default() -> Self {
        Self { bit_rate: 1.0 }
    }
}

impl AudioEffect for BitCrusher {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {

    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        BitCrusherView::new(cx, EffectHandle::<Self>::from(handle));
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Lens, Message, Data)]
struct BitCrusherView {
    #[msg]
    bit_rate: f32,

    #[lens(ignore)]
    #[data(ignore)]
    handle: EffectHandle<BitCrusher>,
}

impl BitCrusherView {
    pub fn new(cx: &mut Context, handle: EffectHandle<BitCrusher>) -> Handle<Self> {
        Self {
            bit_rate: handle.bit_rate,
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            LabeledKnob::new(
                cx,
                Self::bit_rate,
                false,
                0.0..100.0,
            );
        })
    }
}

impl View for BitCrusherView {
    fn element(&self) -> Option<&'static str> {
        Some("bit-crusher")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Bit_rate(val) => {
                self.bit_rate = *val;
                self.handle.bit_rate = *val;
            }
        });
    }
}
