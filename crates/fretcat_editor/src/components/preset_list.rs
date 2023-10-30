use fretcat_serialization::{Preset, PresetCategory};
use nih_plug::vizia::prelude::*;

pub struct PresetList {
    pub(crate) current_category: PresetCategory,
    categories: Vec<PresetCategory>
}

impl PresetList {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            current_category: PresetCategory::User,
            categories: PresetCategory::variants()
        }.build(cx, |cx| {

        })
    }
}

impl View for PresetList {
    fn element(&self) -> Option<&'static str> {
        Some("preset-list")
    }
}