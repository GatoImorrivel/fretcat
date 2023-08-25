use fretcat_effects::{ChainData, ChainCommand};
use fretcat_serialization::Preset;
pub use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct PresetControl {
    pub preset_name: String,
}

pub enum PresetMessage {
    Save,
    NameChange(String)
}

impl PresetControl {
    pub fn new(cx: &mut Context) {
        Self {
            preset_name: "Untitled".to_owned()
        }.build(cx, |cx| {
            cx.add_stylesheet(include_str!("../../css/preset-control.css"))
                .unwrap();
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Textbox::new(cx, Self::preset_name)
                        .on_edit(|ex, val| ex.emit(PresetMessage::NameChange(val)))
                        .class("preset-name");
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
            PresetMessage::NameChange(val) => {
                self.preset_name = val.to_owned();
            }
        });
    }
}
