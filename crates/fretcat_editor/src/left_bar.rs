use nih_plug_vizia::vizia::prelude::*;

use crate::sidebar::{SidebarData, SidebarMessage, SidebarTab};

pub fn left_bar(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/leftbar.css"))
        .unwrap();

    Binding::new(cx, SidebarData::current_tab, |cx, bind| {
        let current_tab = bind.get(cx);
        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Component)),
                |cx| Label::new(cx, "󰡀"),
            )
            .toggle_class("tab-active", current_tab == SidebarTab::Component)
            .class("tab-btn");
            Button::new(
                cx,
                |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Preset)),
                |cx| Label::new(cx, "󰍜"),
            )
            .toggle_class("tab-active", current_tab == SidebarTab::Preset)
            .class("tab-btn");
        });
    });
}
