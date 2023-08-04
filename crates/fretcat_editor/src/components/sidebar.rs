use nih_plug_vizia::vizia::prelude::*;

use super::{card::card_system_init, EffectKind, EFFECT_CARDS};

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
            cx.add_stylesheet(include_str!("../../css/sidebar.css"))
                .unwrap();
            HStack::new(cx, |cx| {
                Binding::new(cx, Sidebar::current_tab, |cx, bind| {
                    let current_tab = bind.get(cx);
                    VStack::new(cx, |cx| {
                        Button::new(
                            cx,
                            |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Effect)),
                            |cx| Label::new(cx, "󰡀"),
                        )
                        .class("tab-btn")
                        .toggle_class("tab-selected-kind", current_tab == SidebarTab::Effect);
                        Button::new(
                            cx,
                            |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Preset)),
                            |cx| Label::new(cx, "󰒓"),
                        )
                        .class("tab-btn")
                        .toggle_class("tab-selected-kind", current_tab == SidebarTab::Preset);
                    })
                    .class("bar");
                    VStack::new(cx, |cx| match current_tab {
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
                            Label::new(cx, "tetas");
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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
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