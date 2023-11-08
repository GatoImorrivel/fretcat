use fretcat_serialization::{Preset, PresetCategory};
use nih_plug::vizia::prelude::*;

use crate::EditorEvent;

use super::PresetMessage;

#[derive(Debug, Clone, Lens)]
pub struct PresetList {
    pub(crate) current_category: PresetCategory,
    categories: Vec<PresetCategory>,
    presets: Vec<Preset>,
}

pub enum PresetListEvent {
    ChangeCategory(PresetCategory),
    Refresh,
}

impl PresetList {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            current_category: PresetCategory::User,
            categories: PresetCategory::variants(),
            presets: Preset::fetch_presets_shallow(),
        }
        .build(cx, |cx| {
            cx.add_listener(|view: &mut PresetList, _, event| {
                event.map(|event, _| match event {
                    PresetListEvent::Refresh => view.presets = Preset::fetch_presets_shallow(),
                    _ => {}
                });
            });

            VStack::new(cx, |cx| {
                let kinds = Self::categories.get(cx);
                let rows = kinds
                    .chunks(2)
                    .map(|chunk: &[PresetCategory]| chunk.to_vec())
                    .collect::<Vec<_>>();

                for row in rows {
                    HStack::new(cx, move |cx| {
                        for kind in row {
                            Button::new(
                                cx,
                                move |ex| ex.emit(PresetListEvent::ChangeCategory(kind.clone())),
                                move |cx| Label::new(cx, &kind.to_string()),
                            )
                            .class("kind-btn")
                            .toggle_class(
                                "kind-selected-btn",
                                Self::current_category.map(move |tab| *tab == kind),
                            );
                        }
                    })
                    .class("kind-btn-row");
                }
            })
            .height(Percentage(15.0))
            .class("kind-btn-wrapper");

            Binding::new(cx, Self::current_category, |cx, current_category| {
                ScrollView::new(cx, 0.0, 0.0, false, false, move |cx| {
                    let current_category = current_category.get(cx);
                    List::new(
                        cx,
                        Self::presets.map(move |presets| {
                            presets
                                .clone()
                                .into_iter()
                                .filter(|preset| preset.get_category() == current_category)
                                .collect::<Vec<_>>()
                        }),
                        move |cx, _, preset| {
                            let preset = preset.get(cx);
                            let preset2 = preset.clone();
                            Button::new(
                                cx,
                                move |ex| ex.emit(EditorEvent::LoadPreset(preset.clone())),
                                move |cx| {
                                    Label::new(cx, preset2.get_name())
                                },
                            ).class("preset-card");
                        },
                    )
                    .width(Stretch(1.0))
                    .row_between(Pixels(10.0));
                })
                .width(Stretch(1.0))
                .height(Percentage(85.0));
            });
        })
    }
}

impl View for PresetList {
    fn element(&self) -> Option<&'static str> {
        Some("preset-list")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            PresetListEvent::ChangeCategory(category) => {
                self.current_category = *category;
            }
            _ => {}
        });
    }
}
