mod chain;
mod common;
pub mod effects;

use std::cell::UnsafeCell;

use nih_plug_vizia::vizia::{
    image::{load_from_memory_with_format, DynamicImage, ImageFormat},
    prelude::*,
};

pub use atomic_refcell::*;
pub use chain::{Chain, ChainCommand, ChainData, ChainHandle};
pub use dyn_clone;
pub use rayon;

pub struct Internal<T: Default> {
    internals: UnsafeCell<T>,
}

pub fn register_fonts(cx: &mut Context) {
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/SymbolsNerdFontMono-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Saturday.otf"));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Montserrat.ttf"));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Get Now.otf"));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Hatch.ttf"));
}

pub fn register_images(cx: &mut Context) {
    let drive = load_from_memory_with_format(
        include_bytes!("../../assets/images/drive-bg.png"),
        ImageFormat::Png,
    )
    .unwrap()
    .thumbnail(1000, 500)
    .brighten(-50)
    .to_rgb8();

    cx.load_image(
        "drive-background",
        DynamicImage::ImageRgb8(drive),
        ImageRetentionPolicy::Forever,
    )
    .build(cx);

    let fuzz = load_from_memory_with_format(
        include_bytes!("../../assets/images/fuzz-bg.png"),
        ImageFormat::Png,
    )
    .unwrap()
    .to_rgb8();

    cx.load_image(
        "fuzz-background",
        DynamicImage::ImageRgb8(fuzz),
        ImageRetentionPolicy::Forever,
    );
}

pub fn register_styles(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/overdrive.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/fuzz.css"))
        .unwrap();
}