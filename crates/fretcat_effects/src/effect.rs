use core::fmt;
use std::any::Any;
use nih_plug_vizia::vizia::prelude::*;
use rand::random;

#[derive(Debug, Clone, Copy, Data, Hash, PartialEq, Eq)]
pub struct Effect(u64);

impl Effect {
    pub fn new() -> Self {
        Self(random())
    }
}

pub trait AudioEffect: fmt::Debug + Send + Sync {
    fn process(&self, _sample: f32) -> f32;
    fn view(&self, cx: &mut Context, handle: &Effect);
    fn height(&self) -> f32;
    fn title(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}