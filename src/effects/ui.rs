use nih_plug::nih_log;
use nih_plug_iced::{slider, Column, Element, Slider};

use super::{EffectMessage, EffectUpdate};
use crate::effects::OverdriveMessage;

pub trait EffectUI {
    fn view(&mut self) -> Element<'_, EffectUpdate>;
    fn update(&mut self, message: EffectMessage);
}

pub struct OverdriveUI {
    id: usize,
    gain: f32,
    gain_slider: slider::State,
}

impl OverdriveUI {
    pub fn new(id: usize, gain: f32) -> Self {
        Self {
            id: id,
            gain: gain,
            gain_slider: slider::State::new()
        }
    }
}

impl EffectUI for OverdriveUI {
    fn update(&mut self, message: EffectMessage) {
        match message {
            EffectMessage::OverdriveMessage(msg) => match msg {
                super::OverdriveMessage::Gain(gain) => self.gain = gain,
            },
            _ => nih_log!("OverdriveUI received invalid message, discarding"),
        }
    }

    fn view(&mut self) -> Element<'_, EffectUpdate> {
        let id = self.id;

        let slider = Slider::new(
            &mut self.gain_slider,
            0.0..=30.0,
            self.gain,
            move |gain| EffectUpdate {
                id,
                message: OverdriveMessage::Gain(gain).into()
            } 
        );

        Column::new()
            .push(slider)
            .into()
    }
}
