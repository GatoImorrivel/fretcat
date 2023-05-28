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
    volume: f32,
    threshold: f32,
    gain_slider: slider::State,
    volume_slider: slider::State,
    threshold_slider: slider::State,
}

impl OverdriveUI {
    pub fn new(id: usize, gain: f32, volume: f32, threshold: f32) -> Self {
        Self {
            id: id,
            gain,
            volume,
            threshold,
            gain_slider: slider::State::new(),
            volume_slider: slider::State::new(),
            threshold_slider: slider::State::new(),
        }
    }
}

impl EffectUI for OverdriveUI {
    fn update(&mut self, message: EffectMessage) {
        match message {
            EffectMessage::OverdriveMessage(msg) => match msg {
                super::OverdriveMessage::Gain(gain) => self.gain = gain,
                super::OverdriveMessage::Volume(volume) => self.volume = volume,
                super::OverdriveMessage::Threshold(threshold) => self.threshold = threshold,
            },
            _ => nih_log!("OverdriveUI received invalid message, discarding"),
        }
    }

    fn view(&mut self) -> Element<'_, EffectUpdate> {
        let id = self.id;

        let gain_slider = Slider::new(
            &mut self.gain_slider,
            1.0..=10.0,
            self.gain,
            move |gain| EffectUpdate {
                id,
                message: OverdriveMessage::Gain(gain).into()
            } 
        );

        let volume_slider = Slider::new(
            &mut self.volume_slider,
            1.0..=10.0,
            self.volume,
            move |volume| EffectUpdate {
                id,
                message: OverdriveMessage::Volume(volume).into()
            } 
        );

        let threshold_slider = Slider::new(
            &mut self.threshold_slider,
            1.0..=10.0,
            self.threshold,
            move |threshold| EffectUpdate {
                id,
                message: OverdriveMessage::Threshold(threshold).into()
            } 
        );

        Column::new()
            .push(gain_slider)
            .push(volume_slider)
            .push(threshold_slider)
            .into()
    }
}
