use nih_plug_vizia::vizia::{
    image::{self, ImageFormat},
    prelude::*,
};

use crate::sidebar::{SidebarMessage, SidebarTab};

pub fn left_bar(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/leftbar.css"))
        .unwrap();

    VStack::new(cx, |cx| {
        Button::new(
            cx,
            |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Preset)),
            |cx| {
                Label::new(cx, "P")
            },
        );
        Button::new(
            cx,
            |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Component)),
            |cx| {
                Label::new(cx, "C")
            },
        );
    });
}
