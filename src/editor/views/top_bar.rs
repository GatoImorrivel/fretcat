use nih_plug_vizia::vizia::{image, prelude::*};

pub fn top_bar(cx: &mut Context) {
    cx.load_image(
        "test.png",
        image::load_from_memory_with_format(
            include_bytes!("../../../res/teste.png"),
            image::ImageFormat::Png,
        ).unwrap(),
        ImageRetentionPolicy::Forever,
    );
    cx.add_stylesheet(include_str!("./top_bar.css")).unwrap();

    HStack::new(cx, |cx| {
        Element::new(cx).class("cock");
    }).class("top-bar");
}
