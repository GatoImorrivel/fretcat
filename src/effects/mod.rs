pub mod overdrive;
mod common;

use core::fmt;
use nih_plug_vizia::vizia::prelude::*;

pub trait Effect: fmt::Debug + Send + Sync {
    fn process(&self, _sample: f32) -> f32;
    fn render(&self, cx: &mut Context);
}