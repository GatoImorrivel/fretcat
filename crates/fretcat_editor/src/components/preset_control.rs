use std::sync::Arc;

use fretcat_effects::{ChainData, Chain};
use fretcat_serialization::Preset;
pub use nih_plug::vizia::prelude::*;

use crate::systems::{MessageEvent, Message};

#[derive(Debug, Clone, Lens)]
pub struct PresetControl {
    pub preset_name: String,
    pub current_preset: Preset,
    color: Color,
}

pub enum PresetMessage {
    New,
    Save,
    Delete,
    ChangePreset(Preset),
    TextChange(String),
    ChangeColor(Color),
}

impl PresetControl {
    pub fn new<L: Lens<Target = Arc<Chain>>>(cx: &mut Context, lens: Option<L>) -> Handle<Self> {
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
                HStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |ex| ex.emit(PresetMessage::Save),
                        |cx| Label::new(cx, "+"),
                    )
                    .class("save-btn");
                    Button::new(
                        cx,
                        |ex| ex.emit(PresetMessage::Save),
                        |cx| Label::new(cx, "󰆓"),
                    )
                    .class("save-btn");
                    Button::new(
                        cx,
                        |ex| ex.emit(PresetMessage::Save),
                        |cx| Label::new(cx, ""),
                    )
                    .class("save-btn");
                }).child_space(Stretch(1.0)).col_between(Stretch(1.0));
            });
        })
    }
}

impl View for PresetControl {
    fn element(&self) -> Option<&'static str> {
        Some("preset-control")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            PresetMessage::New => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.to_owned());

                if self.current_preset != preset {
                    cx.emit(MessageEvent::Warning("There are unsaved changes".to_owned()));
                    return;
                }

                self.current_preset = Preset::default();
                ChainData::as_mut_ex(cx).effects = vec![];
            }
            PresetMessage::Save => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.to_owned());
                self.current_preset.set_name(self.preset_name.to_owned());

                if self.current_preset.already_exists() {
                    cx.emit(MessageEvent::Error("This preset already exists".to_owned()));
                    return;
                }

                if let Ok(_) = preset.save() {
                    cx.emit(MessageEvent::Info("Saved successfully".to_owned()));
                    self.current_preset = preset;
                } else {
                    cx.emit(MessageEvent::Error("Failed to save preset".to_owned()));
                }
            }
            PresetMessage::Delete => {
                let chain = ChainData::chain.get(cx);
                let preset = Preset::from(chain);

                if self.current_preset != preset {
                    cx.emit(MessageEvent::Warning("There are unsaved changes".to_owned()));
                    return;
                }

                self.current_preset = Preset::default();
                ChainData::as_mut_ex(cx).effects = vec![];
            }
            PresetMessage::ChangePreset(incoming_preset) => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.to_owned());

                if self.current_preset != preset {
                    cx.emit(MessageEvent::Warning("There are unsaved changes".to_owned()));
                    return;
                }

                self.current_preset = incoming_preset.clone();
            }
            PresetMessage::TextChange(text) => {
                if text.len() > 0 {
                    self.preset_name = text.to_owned();
                }
            }
            PresetMessage::ChangeColor(color) => {
                self.color = *color;
            }
        });
    }
}
