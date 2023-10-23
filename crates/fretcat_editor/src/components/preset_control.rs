use std::sync::Arc;

use fretcat_effects::{ChainData, Chain, effects::AudioEffect};
use fretcat_serialization::Preset;
pub use nih_plug::vizia::prelude::*;

#[derive(Debug, Clone, Lens)]
pub struct PresetControl {
    pub preset_name: String,
    pub current_preset: Preset,
    pub color: Color,
}

pub enum PresetMessage {
    Save,
    TextChange(String),
    ChangeColor(Color),
}

impl PresetControl {
    pub fn new<L: Lens<Target = Arc<Chain>>>(cx: &mut Context, lens: Option<L>) {
        let current_preset = if let Some(lens) = lens {
            let chain = lens.get(cx);
            Preset::from(chain)
        } else {
            Preset::default()
        };
        Self {
            preset_name: "Untitled".to_owned(),
            color: Color::transparent(),
            current_preset,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Textbox::new_multiline(cx, Self::preset_name, true)
                        .class("preset-name")
                        .on_edit(|cx, str| {
                            cx.emit(PresetMessage::TextChange(str));
                        })
                        .on_submit(|ex, _, _| {
                            ex.emit(PresetMessage::ChangeColor(Color::transparent()));
                        })
                        .on_press_down(|ex| {
                            ex.emit(PresetMessage::ChangeColor(Color::whitesmoke()));
                        })
                        .bind(Self::color, |me, bind| {
                            me.caret_color(bind.0);
                        });
                })
                .class("name-wrapper");
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


            }
            PresetMessage::TextChange(text) => {
                if text.len() > 0 {
                    self.preset_name = text.to_owned();
                    self.current_preset.set_name(text.to_owned());
                }
            }
            PresetMessage::ChangeColor(color) => {
                self.color = *color;
            }
        });
    }
}
