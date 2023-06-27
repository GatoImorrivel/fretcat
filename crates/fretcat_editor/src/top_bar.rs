use nih_plug_vizia::vizia::{
    image::{self, ImageFormat},
    prelude::*,
};

pub fn top_bar(cx: &mut Context) {
    cx.load_image(
        "logo.png",
        image::load_from_memory_with_format(include_bytes!("res/logo.png"), ImageFormat::Png)
            .unwrap(),
        ImageRetentionPolicy::Forever,
    );

    cx.add_stylesheet(include_style!("../css/topbar.css"))
        .unwrap();

    Element::new(cx).class("logo");
}
