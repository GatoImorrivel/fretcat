use fretcat_effects::{
    effects::{Gain, PreFX},
    ChainData,
};
use fretcat_serialization::PresetCategory;
use nih_plug::{util::MINUS_INFINITY_DB, vizia::prelude::*};
use strum::IntoEnumIterator;

use super::{accordion::Accordion, audio_slider::AudioSlider, EffectKind, EFFECT_CARDS};

const KIND_PER_ROW: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum SidebarTab {
    Effect,
    Preset,
}

#[derive(Lens)]
pub struct Sidebar {
    pub current_tab: SidebarTab,
    pub selected_kind: EffectKind,
}

enum SidebarMessage {
    ChangeTab(SidebarTab),
    ChangeSelectedKind(EffectKind),
}

impl Sidebar {
    pub fn new(cx: &mut Context) {
        Self {
            current_tab: SidebarTab::Effect,
            selected_kind: EffectKind::Distortion,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Effect)),
                        |cx| Label::new(cx, "󰡀"),
                    )
                    .class("tab-btn")
                    .bind(Self::current_tab, |mut view, bind| {
                        let current_tab = bind.get(view.context());
                        view.toggle_class("tab-selected-kind", current_tab == SidebarTab::Effect);
                    });

                    Button::new(
                        cx,
                        |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Preset)),
                        |cx| Label::new(cx, ""),
                    )
                    .class("tab-btn")
                    .bind(Self::current_tab, |mut view, bind| {
                        let current_tab = bind.get(view.context());
                        view.toggle_class("tab-selected-kind", current_tab == SidebarTab::Preset);
                    });

                    VStack::new(cx, |cx| {
                        AudioSlider::new(
                            cx,
                            200.0,
                            ChainData::chain.map(|chain| chain.in_avg_amplitude),
                            |ex, val| {
                                let chain = ChainData::as_mut_ex(ex);
                                let prefx = chain
                                    .get_pre_fx::<Gain>(&PreFX("in_gain"))
                                    .expect("No in gain");
                                let gain = if val > -60.0 { val } else { 0.0 };
                                prefx.gain_in_db = gain;
                            },
                        );
                        AudioSlider::new(
                            cx,
                            200.0,
                            ChainData::chain.map(|chain| chain.out_avg_amplitude),
                            |ex, val| {
                                let chain = ChainData::as_mut_ex(ex);
                                let postfx = chain
                                    .get_pre_fx::<Gain>(&PreFX("out_gain"))
                                    .expect("No out gain");
                                let gain = if val > -60.0 { val } else { 0.0 };
                                postfx.gain_in_db = gain;
                            },
                        );
                    })
                    .child_space(Stretch(1.0))
                    .row_between(Stretch(1.0))
                    .height(Stretch(1.0))
                    .width(Stretch(1.0));
                })
                .class("bar");
                Binding::new(cx, Sidebar::current_tab, |cx, bind| {
                    VStack::new(cx, |cx| match bind.get(cx) {
                        SidebarTab::Effect => {
                            Binding::new(cx, Sidebar::selected_kind, |cx, bind| {
                                VStack::new(cx, |cx| {
                                    let selection = bind.get(cx);
                                    let kinds = EffectKind::variants();
                                    let kind_rows: Vec<Vec<EffectKind>> = kinds
                                        .chunks(KIND_PER_ROW)
                                        .map(|chunk| chunk.to_vec())
                                        .collect();

                                    for row in kind_rows {
                                        HStack::new(cx, |cx| {
                                            for kind in row {
                                                let c1 = kind.clone();
                                                let c2 = kind.clone();
                                                Button::new(
                                                    cx,
                                                    move |ex| {
                                                        ex.emit(SidebarMessage::ChangeSelectedKind(
                                                            c1,
                                                        ))
                                                    },
                                                    |cx| Label::new(cx, &kind.clone().to_string()),
                                                )
                                                .class("kind-btn")
                                                .toggle_class("kind-selected-btn", c2 == selection);
                                            }
                                        })
                                        .class("picker-row");
                                    }
                                })
                                .class("picker-wrapper");
                                VStack::new(cx, |cx| {
                                    let cards = EFFECT_CARDS.get(&bind.get(cx)).unwrap();
                                    VStack::new(cx, |cx| {
                                        cards.iter().for_each(|card| {
                                            card.render(cx);
                                        });
                                    })
                                    .class("cards-wrapper");
                                });
                            });
                        }
                        SidebarTab::Preset => {
                            let preset_kinds = PresetCategory::iter()
                                .map(|category| category.to_string())
                                .collect::<Vec<_>>();
                            preset_kinds.into_iter().for_each(|kind| {
                                Chip::new(cx, &kind);
                            });
                        }
                    })
                    .class("content");
                });
            })
            .class("sidebar");
        });
    }
}

impl View for Sidebar {
    fn element(&self) -> Option<&'static str> {
        Some("sidebar")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            SidebarMessage::ChangeTab(tab) => {
                self.current_tab = *tab;
            }
            SidebarMessage::ChangeSelectedKind(kin) => {
                self.selected_kind = *kin;
            }
        });
    }
}
