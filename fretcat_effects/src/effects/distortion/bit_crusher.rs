use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BitCrusher {
    bit_rate: f32,
    sample_rate: f32,
}

impl Default for BitCrusher {
    fn default() -> Self {
        Self {
            sample_rate: 44100.0,
            bit_rate: 44100.0,
        }
    }
}

impl BitCrusher {
    #[inline]
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    #[inline]
    pub fn bit_rate(&self) -> f32 {
        self.bit_rate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    #[inline]
    pub fn set_bit_rate(&mut self, bit_rate: f32) {
        self.bit_rate = bit_rate;
    }

    #[inline]
    fn step_size(&self) -> usize {
        (self.sample_rate() / self.bit_rate()) as usize
    }
}

impl AudioEffect for BitCrusher {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        if self.sample_rate() != transport.sample_rate {
            self.set_sample_rate(transport.sample_rate);
        }

        let step_size = self.step_size();

        let mut sample_index = 0;
        let buffer_size = input_buffer.len();

        while sample_index < buffer_size {
            let first_index = sample_index;
            let limit_index = (sample_index + step_size).min(buffer_size);

            while sample_index < limit_index {
                input_buffer.process_channel(|channel| {
                    let value = channel[first_index];
                    channel[sample_index] = value;
                });
                sample_index += 1;
            }
        }
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        BitCrusherView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
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
            bit_rate: (handle.bit_rate / handle.sample_rate) * 100.0,
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            NamedKnob::new(cx, "Amount", Self::bit_rate, false, 0.0..100.0)
                .on_changing(|ex, val| ex.emit(Message::Bit_rate(val)));
            Label::new(cx, "BIT CRUSHER").class("effect-title");
        })
    }
}

impl View for BitCrusherView {
    fn element(&self) -> Option<&'static str> {
        Some("bit-crusher")
    }

    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Bit_rate(val) => {
                self.bit_rate = *val;
                self.handle.bit_rate = (*val / 100.0) * self.handle.sample_rate;
            }
        });
    }
}
