use std::{usize, rc::Rc, sync::Arc};

mod effect_tab;

use fretcat_effects::{EffectKind, effect::Effect};
use nih_plug_vizia::vizia::prelude::*;

use self::effect_tab::effect_tab;

#[derive(Debug, Lens, Clone)]
struct Sidebar {
    tabs: Vec<&'static str>,
    effect_kinds: Vec<EffectKind>,
    selected_kind: usize,
}

enum SidebarEvent {
    KindChange(usize),
}

impl Model for Sidebar {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            SidebarEvent::KindChange(i) => {
                self.selected_kind = *i;
            }
        })
    }
}

pub fn sidebar(cx: &mut Context) {
    Sidebar {
        tabs: vec!["Effects", "Presets"],
        effect_kinds: EffectKind::variants(),
        selected_kind: 0,
    }
    .build(cx);

    cx.add_stylesheet(include_str!("../css/sidebar.css"))
        .unwrap();

    VStack::new(cx, |cx| {
        TabView::new(cx, Sidebar::tabs, |cx, tab| match tab.get(cx) {
            "Effects" => TabPair::new(
                move |cx| {
                    Label::new(cx, tab);
                },
                |cx| {
                    effect_tab(cx);
                },
            ),

            "Presets" => TabPair::new(
                move |cx| {
                    Label::new(cx, tab);
                },
                |cx| {
                    preset_tab(cx);
                },
            ),
            _ => unreachable!(),
        })
        .class("tabs");
    })
    .class("sidebar");
}


fn preset_tab(cx: &mut Context) {}
