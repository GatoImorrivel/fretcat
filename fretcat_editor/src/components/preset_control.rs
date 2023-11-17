use fretcat_effects::{
    effects::{NoiseGate, PreFX},
    ChainData,
};
use fretcat_serialization::Preset;
pub use nih_plug::vizia::prelude::*;

use crate::{
    systems::{Message, MessageEvent},
    EditorData, EditorEvent,
};

use super::PresetListEvent;

#[derive(Debug, Clone, Lens)]
pub struct PresetControl {
    pub noise_gate: f32,
    color: Color,
}

pub enum PresetMessage {
    New,
    Save,
    Delete,
    TextChange(String),
    ChangeColor(Color),
    NoiseGate(f32),
}

impl PresetControl {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            color: Color::transparent(),
            noise_gate: 0.0,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Textbox::new_multiline(
                        cx,
                        EditorData::current_preset.map(|p| p.lock().unwrap().get_name().to_owned()),
                        true,
                    )
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
                        |ex| ex.emit(PresetMessage::New),
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
                        |ex| ex.emit(PresetMessage::Delete),
                        |cx| Label::new(cx, ""),
                    )
                    .class("save-btn");
                })
                .child_space(Stretch(1.0))
                .col_between(Stretch(1.0));
            });
        })
    }

    pub fn discard_changes(cx: &mut Context, current: Preset) {
        Button::new(
            cx,
            move |ex| {
                ex.emit(EditorEvent::LoadPreset(current.clone()));
                ex.emit(MessageEvent::ClearAll);
            },
            |cx| Label::new(cx, "Discard changes?").color(Color::whitesmoke()),
        );
    }

    pub fn overwrite(cx: &mut Context, current: Preset) {
        Button::new(
            cx,
            move |ex| {
                ex.emit(MessageEvent::ClearAll);
                if let Ok(_) = current.save() {
                    ex.emit(EditorEvent::LoadPreset(current.clone()));
                    ex.emit(MessageEvent::Info("Overwriten succesfully".to_owned()));
                    ex.emit(PresetListEvent::Refresh);
                } else {
                    ex.emit(MessageEvent::Error("Failed to overwrite".to_owned()));
                }
            },
            |cx| Label::new(cx, "Overwrite?").color(Color::whitesmoke()),
        );
    }
}

impl View for PresetControl {
    fn element(&self) -> Option<&'static str> {
        Some("preset-control")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| {
            let original_data = EditorData::current_preset.get(cx);
            let mut current = Preset::from(ChainData::chain.get(cx));
            current.set_name(original_data.lock().unwrap().get_name().to_owned());

            match event {
                PresetMessage::New => {
                    if *original_data.lock().unwrap() != current || !current.already_exists() {
                        cx.emit(MessageEvent::Custom(
                            Message::make_warning("Unsaved changes").with_custom_content(
                                move |cx, _| Self::discard_changes(cx, Preset::default()),
                            ),
                        ))
                    } else {
                        cx.emit(EditorEvent::LoadPreset(Preset::default()));
                        cx.emit(MessageEvent::ClearAll);
                    }
                }
                PresetMessage::Save => {
                    if current.already_exists() {
                        cx.emit(MessageEvent::Custom(
                            Message::make_warning("There is a preset with the same name")
                                .with_custom_content(move |cx, _| {
                                    Self::overwrite(cx, current.clone())
                                }),
                        ))
                    } else {
                        if let Ok(_) = current.save() {
                            cx.emit(EditorEvent::LoadPreset(current.clone()));
                            cx.emit(MessageEvent::Info("Saved successfully".to_owned()));
                            cx.emit(PresetListEvent::Refresh);
                        } else {
                            cx.emit(MessageEvent::Error("Failed to save preset".to_owned()));
                        }
                    }
                }
                PresetMessage::Delete => match current.delete() {
                    Ok(_) => {
                        cx.emit(MessageEvent::Info("Deleted preset successfully".to_owned()));
                        cx.emit(EditorEvent::LoadPreset(Preset::default()));
                        cx.emit(PresetListEvent::Refresh);
                    }
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::NotFound => {
                            cx.emit(MessageEvent::Error(
                                "Preset has not been saved yet".to_owned(),
                            ));
                        }
                        _ => {
                            cx.emit(MessageEvent::Error("Failed to delete preset".to_owned()));
                        }
                    },
                },
                PresetMessage::TextChange(text) => {
                    if text.len() > 0 {
                        let p = EditorData::current_preset.get(cx);
                        p.lock().unwrap().set_name(text.to_owned());
                    }
                }
                PresetMessage::ChangeColor(color) => {
                    self.color = *color;
                }
                PresetMessage::NoiseGate(val) => {
                    self.noise_gate = *val;
                    ChainData::as_mut_ex(cx)
                        .get_pre_fx::<NoiseGate>(&PreFX("noise_gate"))
                        .unwrap()
                        .set_threshold(*val);
                }
            }
        });
    }
}
