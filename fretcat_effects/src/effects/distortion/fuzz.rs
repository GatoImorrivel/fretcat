use std::f32::consts::PI;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fuzz {
    pub drive: f32,
    pub tone: f32,
    pub boost: f32,
    pub fuzziness: f32,
    max_freq_hz: f32,
    min_freq_hz: f32,
    filter: [ButterLowpass; NUM_CHANNELS],
    pre_filter: [DCBlock; NUM_CHANNELS],
}

impl PartialEq for Fuzz {
    fn eq(&self, other: &Self) -> bool {
        self.drive == other.drive && self.tone == other.tone && self.boost == other.boost
    }
}

impl Default for Fuzz {
    fn default() -> Self {
        let min_freq_hz = 1000.0;
        Self {
            drive: 1.0,
            tone: min_freq_hz,
            boost: 1.0,
            fuzziness: 1.0,
            max_freq_hz: 2000.0,
            min_freq_hz,
            filter: [ButterLowpass::new(min_freq_hz); NUM_CHANNELS],
            pre_filter: [DCBlock::new(500.0); NUM_CHANNELS],
        }
    }
}

impl AudioEffect for Fuzz {
    fn process(&mut self, input_buffer: &mut Frame, _transport: &nih_plug::prelude::Transport) {
        if self.filter[0].sample_rate() != _transport.sample_rate {
            self.filter[0].set_sample_rate(_transport.sample_rate);
            self.filter[1].set_sample_rate(_transport.sample_rate);
        }

        if self.pre_filter[0].sample_rate() != _transport.sample_rate {
            self.pre_filter[0].set_sample_rate(_transport.sample_rate);
            self.pre_filter[1].set_sample_rate(_transport.sample_rate);
        }

        input_buffer.process_individual(|left, right| {
            *left = self.pre_filter[0].tick(*left);
            *right = self.pre_filter[1].tick(*right);

            let offset_l = left.abs() * (self.fuzziness / 100.0);
            let offset_r = right.abs() * (self.fuzziness / 100.0);

            *left += offset_l;
            *right += offset_r;

            let gain = ((self.boost / 100.0) * 100.0) + 1.0;

            *left *= gain;
            *right *= gain;

            let a = (((self.drive + 1.0) / 101.0) * (PI / 2.0)).sin();
            let k = 2.0 * a / (1.0 - a);

            let drive_l = (1.0 + k) * *left / (1.0 + k * left.abs());
            let drive_r = (1.0 + k) * *right / (1.0 + k * right.abs());

            *left = drive_l;
            *right = drive_r;

            *left = self.filter[0].tick(*left);
            *right = self.filter[1].tick(*right);
        });
    }

    fn view(&self, cx: &mut Context, handle: EffectHandle<dyn AudioEffect>) {
        FuzzView::new(cx, EffectHandle::<Self>::from(handle)).class("base-effect");
    }

    fn height(&self) -> f32 {
        100.0
    }
}

#[derive(Debug, Clone, Data, Lens, Message)]
pub struct FuzzView {
    #[msg]
    pub drive: f32,
    #[msg]
    pub tone: f32,
    #[msg]
    pub boost: f32,
    #[msg]
    pub fuzziness: f32,

    #[lens(ignore)]
    #[data(ignore)]
    handle: EffectHandle<Fuzz>,
}

impl FuzzView {
    pub fn new(cx: &mut Context, handle: EffectHandle<Fuzz>) -> Handle<Self> {
        Self {
            drive: handle.drive,
            tone: handle.tone,
            boost: handle.boost,
            fuzziness: handle.fuzziness,
            handle: handle.clone(),
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                NamedKnob::new(cx, "Drive", Self::drive, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Drive(val)))
                    .class("gain-knob");
                NamedKnob::new(cx, "Boost", Self::boost, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Boost(val)))
                    .class("volume-knob");
                NamedKnob::new(cx, "Fuzziness", Self::fuzziness, false, 0.0..100.0)
                    .on_changing(|ex, val| ex.emit(Message::Fuzziness(val)))
                    .class("fuzz-knob");
                NamedKnob::new(
                    cx,
                    "Tone",
                    Self::tone,
                    false,
                    handle.min_freq_hz..handle.max_freq_hz,
                )
                .on_changing(|ex, val| ex.emit(Message::Tone(val)))
                .class("tone-knob");
                Label::new(cx, "FUZZ").class("effect-title");
            });
        })
    }
}

impl View for FuzzView {
    fn element(&self) -> Option<&'static str> {
        Some("fuzz")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Drive(val) => {
                self.drive = *val;
                self.handle.drive = *val;
            }
            Message::Tone(val) => {
                self.tone = *val;
                self.handle.tone = *val;
                self.handle.filter.iter_mut().for_each(|filter| {
                    filter.set_cutoff(
                        *val,
                    );
                });
            }
            Message::Boost(val) => {
                self.boost = *val;
                self.handle.boost = *val;
            }
            Message::Fuzziness(val) => {
                self.fuzziness = *val;
                self.handle.fuzziness = *val;
            }
        });
    }
}

