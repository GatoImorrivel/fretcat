use fretcat_serialization::Preset;
use nih_plug::vizia::prelude::*;

pub struct PresetList;

impl PresetList {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {

        }.build(cx, |cx| {

        })
    }
}

impl View for PresetList {
    fn element(&self) -> Option<&'static str> {
        Some("preset-list")
    }
}