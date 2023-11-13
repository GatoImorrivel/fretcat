use crate::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudioReverb {
    pub wet: f32,
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

impl PartialEq for StudioReverb {
    fn eq(&self, other: &Self) -> bool {
        self.wet == other.wet && self.size == other.size
    }
}

impl AudioEffect for StudioReverb {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        if transport.sample_rate != self.reverb.sample_rate() {
            nih_plug::util::permit_alloc(|| {
                self.reverb = Freeverb::new(transport.sample_rate as usize);
                self.reverb.set_wet(self.wet);
                self.reverb.set_room_size(self.size);
            });
        }

        input_buffer.process_individual(|left, right| {
            let (reverbed_l, reverbed_r) = self.reverb.tick((*left, *right));
            *left = ((1.0 - self.wet) * *left) + reverbed_l;
            *right = ((1.0 - self.wet) * *right) + reverbed_r;
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        StudioReverbView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
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
    handle: EffectHandle<StudioReverb>,
}

impl StudioReverbView {
    pub fn new(cx: &mut Context, handle: EffectHandle<StudioReverb>) -> Handle<Self> {
        Self {
            size: handle.size * 100.0,
            wet: handle.wet * 100.0,
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Room Size", Self::size, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Size(val)));
                NamedKnob::new(cx, "Wet", Self::wet, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Wet(val)));
            });
        })
    }
}

impl View for StudioReverbView {
    fn element(&self) -> Option<&'static str> {
        Some("studio-reverb")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            Message::Size(val) => {
                self.size = *val;
                self.handle.size = *val / 100.0;
                self.handle.reverb.set_room_size(*val / 100.0);
            }
            Message::Wet(val) => {
                self.wet = *val;
                self.handle.wet = *val / 100.0;
                self.handle.reverb.set_wet(*val / 100.0);
            }
        });
    }
}
