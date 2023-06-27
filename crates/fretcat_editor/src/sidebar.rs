use nih_plug_vizia::vizia::prelude::*;

#[derive(Debug, Lens, Clone)]
struct Sidebar {
    tabs: Vec<&'static str>,
}

impl Model for Sidebar {}

pub fn sidebar(cx: &mut Context) {
    Sidebar {
        tabs: vec!["Effects", "Presets"],
    }
    .build(cx);

    VStack::new(cx, |cx| {
        TabView::new(cx, Sidebar::tabs, |cx, tab| match tab.get(cx) {
            "Effects" => TabPair::new(
                move |cx| {
                    Label::new(cx, tab);
                },
                |cx| {
                    Label::new(cx, "effects");
                },
            ),

            "Presets" => TabPair::new(
                move |cx| {
                    Label::new(cx, tab);
                },
                |cx| {
                    Label::new(cx, "presets");
                },
            ),
            _ => unreachable!(),
        }).class("tabs");
    }).class("sidebar");
}
