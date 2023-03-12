mod macros;

use nih_plug_iced::Element;

use crate::create_messages;

create_messages!(Overdrive { gain: f32 }, Highpass { freq: f32 });

pub enum EffectCategory {
    Distortion,
    Filter,
}
pub struct Effect {
    name: String,
    kind: EffectCategory,
    state: EffectState,
    algorithm: fn(&EffectState, f32) -> f32,
}

impl Effect {
    pub fn info(&self) -> (&String, &EffectCategory) {
        (&self.name, &self.kind)
    }

    pub fn process(&self, sample: f32) -> f32 {
        (self.algorithm)(&self.state, sample)
    }
}

pub fn make_overdrive() -> Effect {
    Effect {
        name: "Overdrive".to_owned(),
        kind: EffectCategory::Distortion,
        state: EffectState::Overdrive { gain: 2.0 },
        algorithm: move |state, sample| match state {
            EffectState::Overdrive { gain } => sample * gain,
            _ => sample,
        },
    }
}
