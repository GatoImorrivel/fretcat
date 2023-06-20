use nih_plug::nih_log;
use nih_plug_iced::{slider, Column, Slider};

use super::Effect;

#[derive(Debug, Clone, Copy, Default)]
pub struct Overdrive {
    pub(crate) gain: f32,
    gain_state: slider::State,
}

impl Effect for Overdrive {
    fn process(&self, sample: f32) -> f32 {
        sample
    }

    fn render(&mut self) -> nih_plug_iced::Element<'_, ()> {
        let ptr = self as *mut Self;

        let slider = Slider::new(&mut self.gain_state, 0.0..=1.0, self.gain, move |val| {
            unsafe {
                nih_log!("BOlas");
                (*ptr).gain = val;
            }
        })
        .step(0.1);

        Column::new().push(slider).into()
    }
}
