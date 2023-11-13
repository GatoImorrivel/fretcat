use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Gain {
    pub gain_in_db: f32,
}

impl Default for Gain {
    fn default() -> Self {
        Self { gain_in_db: 0.0 }
    }
}

impl AudioEffect for Gain {
    fn process(&mut self, input_buffer: &mut Frame, _transport: &nih_plug::prelude::Transport) {
        input_buffer.process_both(|sample| {
            *sample *= db_to_gain_fast(self.gain_in_db);
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        GainView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
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
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Gain", Self::gain, false, -60.0..20.0)
                    .on_changing(|ex, val| ex.emit(Message::Gain(val)));
                Label::new(cx, "GAIN BOOSTER").class("effect-title");
            });
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
