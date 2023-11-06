use std::sync::{Arc, Mutex};

use fretcat_effects::{
    effects::{NoiseGate, PreFX},
    Chain, ChainCommand, ChainData,
};
use fretcat_serialization::Preset;
pub use nih_plug::vizia::prelude::*;

use crate::systems::{Message, MessageEvent};

use super::labeled_knob::{LabeledKnob, LabeledKnobModifier};

#[derive(Debug, Clone, Lens)]
pub struct PresetControl {
    pub preset_name: Arc<Mutex<String>>,
    pub current_preset: Arc<Mutex<Preset>>,
    pub noise_gate: f32,
    color: Color,
}

pub enum PresetMessage {
    New,
    Save,
    Delete,
    ChangePreset(Preset),
    TextChange(String),
    ChangeColor(Color),
    NoiseGate(f32),
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
            preset_name: Arc::new(Mutex::new("Untitled".to_owned())),
            color: Color::transparent(),
            current_preset: Arc::new(Mutex::new(current_preset)),
            noise_gate: 0.0,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Textbox::new_multiline(
                        cx,
                        Self::preset_name.map(|lock| lock.lock().unwrap().clone()),
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
                LabeledKnob::new(
                    cx,
                    0.0,
                    false,
                    -20.0..20.0,
                    super::labeled_knob::LabelSide::Left,
                    "Noise gate",
                );
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

    fn unsaved_changes(
        cx: &mut Context,
        index: usize,
        name: Arc<Mutex<String>>,
        current: Arc<Mutex<Preset>>,
        new: Preset,
    ) {
        Button::new(
            cx,
            move |ex| {
                let mut current = current.lock().unwrap();
                *current = new.clone();
                *name.lock().unwrap() = current.get_name().to_owned();
                ex.emit(ChainCommand::Load(new.clone().into()));
                ex.emit(MessageEvent::ClearAll);
            },
            |cx| Label::new(cx, "Discard changes?").color(Color::whitesmoke()),
        );
    }

    fn overwrite(
        cx: &mut Context,
        index: usize,
        name: Arc<Mutex<String>>,
        current: Arc<Mutex<Preset>>,
        new: Preset,
    ) {
        Button::new(
            cx,
            move |ex| {
                let mut current = current.lock().unwrap();
                *name.lock().unwrap() = new.get_name().to_owned();
                *current = new.clone();
                if let Ok(_) = current.save() {
                    ex.emit(MessageEvent::Info("Overwriten succesfully".to_owned()));
                } else {
                    ex.emit(MessageEvent::Error("Failed to overwrite".to_owned()));
                }
                ex.emit(MessageEvent::ClearAll);
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
        event.map(|event, _| match event {
            PresetMessage::New => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.lock().unwrap().clone());

                if *self.current_preset.lock().unwrap() != preset {
                    let name = self.preset_name.clone();
                    let current = self.current_preset.clone();
                    cx.emit(MessageEvent::Custom(
                        Message::make_warning("There are unsaved changes").with_custom_content(
                            move |cx, index| {
                                Self::unsaved_changes(
                                    cx,
                                    index,
                                    name.clone(),
                                    current.clone(),
                                    Preset::default(),
                                )
                            },
                        ),
                    ));
                    return;
                }

                let default = Preset::default();
                *self.current_preset.lock().unwrap() = default.clone();
                *self.preset_name.lock().unwrap() = default.get_name().to_owned();
                cx.emit(ChainCommand::Clear);
            }
            PresetMessage::Save => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.lock().unwrap().to_owned());

                if preset.already_exists() {
                    let current = self.current_preset.clone();
                    let name = self.preset_name.clone();
                    let new = preset.clone();
                    cx.emit(MessageEvent::Custom(
                        Message::make_warning("This preset already exists").with_custom_content(
                            move |cx, index| {
                                Self::overwrite(
                                    cx,
                                    index,
                                    name.clone(),
                                    current.clone(),
                                    new.clone(),
                                )
                            },
                        ),
                    ));
                    return;
                }

                if let Ok(_) = preset.save() {
                    cx.emit(MessageEvent::Info("Saved successfully".to_owned()));
                    *self.preset_name.lock().unwrap() = preset.get_name().to_owned();
                    *self.current_preset.lock().unwrap() = preset;
                } else {
                    cx.emit(MessageEvent::Error("Failed to save preset".to_owned()));
                }
            }
            PresetMessage::Delete => {
                let mut current = self.current_preset.lock().unwrap();
                if let Ok(_) = current.delete() {
                    cx.emit(MessageEvent::Info("Deleted succesfully".to_owned()));
                } else {
                    cx.emit(MessageEvent::Error("Failed to delete preset".to_owned()));
                    return;
                }

                let default = Preset::default();
                *current = default.clone();
                *self.preset_name.lock().unwrap() = default.get_name().to_owned();
                cx.emit(ChainCommand::Clear);
            }
            PresetMessage::ChangePreset(incoming_preset) => {
                let chain = ChainData::chain.get(cx);
                let mut preset = Preset::from(chain);
                preset.set_name(self.preset_name.lock().unwrap().clone());

                if *self.current_preset.lock().unwrap() != preset {
                    let current = self.current_preset.clone();
                    let name = self.preset_name.clone();
                    let new = incoming_preset.clone();
                    cx.emit(Event::new(MessageEvent::Custom(
                        Message::make_warning("Unsaved changes").with_custom_content(
                            move |cx, index| {
                                Self::unsaved_changes(
                                    cx,
                                    index,
                                    name.clone(),
                                    current.clone(),
                                    new.clone(),
                                )
                            },
                        ),
                    )));
                    return;
                }

                *self.current_preset.lock().unwrap() = incoming_preset.clone();
            }
            PresetMessage::TextChange(text) => {
                if text.len() > 0 {
                    *self.preset_name.lock().unwrap() = text.to_owned();
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
        });
    }
}
