use nih_plug_iced::{slider, Column, Element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EffectState {
    Overdrive(Overdrive),
}

pub trait Effect {
    type Message;

    fn process(&self, sample: f32) -> f32;
    fn view(&mut self) -> Element<'_, Self::Message>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Overdrive {
    gain: f32,

    #[serde(skip)]
    gain_slider_state: slider::State,
}

#[derive(Debug, Clone, Copy)]
pub enum OverdriveMessages {
    GainChange(f32),
}

impl Effect for Overdrive {
    type Message = OverdriveMessages;

    fn process(&self, sample: f32) -> f32 {
        self.gain * sample
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .push(
                slider::Slider::new(&mut self.gain_slider_state, -30.0..=30.0, self.gain, OverdriveMessages::GainChange)
            )
            .into()
    }
}
