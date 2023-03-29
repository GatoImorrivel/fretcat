pub mod chain;

#[macro_use]
mod macros;

use nih_plug_iced::{slider, Column, Element};
use serde::{Deserialize, Serialize};

use rand::{Rng};

use self::macros::*;


// Base effect trait that defines what which effects has
pub trait Effect<M> {
    fn process(&self, sample: f32) -> f32;
    fn view(&mut self) -> Element<'_, M>;
    fn update(&mut self, message: M);
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Overdrive {
    gain: f32,

    #[serde(skip)]
    gain_slider_state: slider::State,
}

#[derive(Debug, Clone, Copy)]
pub enum OverdriveMessage {
    GainChange(f32),
}

impl Default for Overdrive {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            ..Default::default()
        }
    }
}


impl Effect<OverdriveMessage> for Overdrive {
    fn process(&self, sample: f32) -> f32 {
        self.gain * sample
    }

    fn view(&mut self) -> Element<'_, OverdriveMessage> {
        Column::new()
            .push(slider::Slider::new(
                &mut self.gain_slider_state,
                -30.0..=30.0,
                self.gain,
                OverdriveMessage::GainChange,
            ))
            .into()
    }

    fn update(&mut self, message: OverdriveMessage) {
        match message {
            OverdriveMessage::GainChange(gain) => self.gain = gain,
        }
    }
}

effects!(Overdrive);