
use nih_plug_iced::{slider, Column, Element, Text, Color};

pub trait EffectUI: Send + Sync {
    type Message;
    fn view(&mut self) -> Element<'_, Self::Message>;
    fn update(&mut self, message: Self::Message);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OverdriveUI {
    gain: f32,
    gain_slider_state: slider::State,
}

#[derive(Debug, Clone, Copy)]
pub enum OverdriveMessage {
    GainChange(f32),
}


impl EffectUI for OverdriveUI {
    type Message = OverdriveMessage; 

    fn view(&mut self) -> Element<'_, OverdriveMessage> {
        Column::new()
            .push(Text::new("Gain").color(Color::WHITE))
            .push(slider::Slider::new(
                &mut self.gain_slider_state,
                0.0..=5.0,
                self.gain,
                OverdriveMessage::GainChange,
            ).step(0.5))
            .into()
    }

    fn update(&mut self, message: OverdriveMessage) {
        match message {
            OverdriveMessage::GainChange(gain) => self.gain = gain,
        }
    }
}

ui_message