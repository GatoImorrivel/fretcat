use nih_plug_vizia::vizia::{image, prelude::*};

pub fn top_bar(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/top_bar.css")).unwrap();

    HStack::new(cx, |cx| {
        Element::new(cx).class("cock");
    }).class("top-bar");
}
