use nih_plug_iced::Element;

pub trait EffectUI {
    type Message;
    
    fn view<'a>(&mut self) -> Element<'a, Self::Message>;
    fn update(&mut self, message: Self::Message);
}

pub mod overdrive {
    use iced::widget::{slider, column};
    use nih_plug_iced::{slider, Column, Slider};

    use super::EffectUI;

    pub struct Overdrive {
        gain: f32,
        gain_slider_state: slider::State
    }

    impl Overdrive {
        pub fn new() -> Self {
            Self {
                gain: 0.0,
                gain_slider_state: slider::State::new()
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        GainChange(f32)
    }

    impl EffectUI for Overdrive {
        type Message = Message;

        fn update(&mut self, message: Self::Message) {
           match message {
                Message::GainChange(gain) => self.gain = gain,
           } 
        }

        fn view<'a>(&mut self) -> nih_plug_iced::Element<'a, Self::Message> {
            column( 
                slider(-30.0..=30.0, 0.0, Message::GainChange)
            ).into()
        }
    }
}