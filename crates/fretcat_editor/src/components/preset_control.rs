use fretcat_effects::ChainData;
use fretcat_serialization::Preset;
pub use fretcat_common::vizia::prelude::*;

#[derive(Debug, Clone, Lens)]
pub struct PresetControl {
    pub preset_name: String
}

pub enum PresetMessage {
    Save
}

impl PresetControl {
    pub fn new(cx: &mut Context) {
        Self { preset_name: "Untitled".to_owned() }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Textbox::new_multiline(cx, Self::preset_name, true).class("preset-name");
                }).class("name-wrapper");
                Button::new(
                    cx,
                    |ex| ex.emit(PresetMessage::Save),
                    |cx| Label::new(cx, "󰆓"),
                )
                .class("save-btn");
            });
        });
    }
}

impl View for PresetControl {
    fn element(&self) -> Option<&'static str> {
        Some("preset-control")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            PresetMessage::Save => {
                let chain = ChainData::chain.get(cx);

                let preset = Preset::from(chain);

                preset.save();
            }
        });
    }
}
