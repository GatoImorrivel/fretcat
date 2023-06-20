use std::fmt::Debug;

use nih_plug_iced::Element;

pub mod chain;

pub mod overdrive;

pub trait Effect: Debug + Send + Sync {
    fn process(&self, sample: f32) -> f32;
    fn render(&mut self) -> Element<()>;
}