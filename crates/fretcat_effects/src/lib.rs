mod chain;
mod common;
pub mod effects;

use nih_plug::vizia::{
    image::{load_from_memory_with_format, DynamicImage, ImageFormat, imageops},
    prelude::*,
};

pub use atomic_refcell::*;
pub use chain::{Chain, ChainCommand, ChainData, NUM_CHANNELS};
pub use dyn_clone;

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

    let studio_reverb = load_from_memory_with_format(
        include_bytes!("../../assets/images/reverb-bg.png"),
        ImageFormat::Png,
    )
    .unwrap();

    let width = studio_reverb.width();
    let height = studio_reverb.height();

    let studio_reverb = studio_reverb.resize_to_fill(width * 2, height, imageops::FilterType::Nearest).to_rgb8();

    cx.load_image(
        "reverb-background",
        DynamicImage::ImageRgb8(studio_reverb),
        ImageRetentionPolicy::Forever,
    );
}

pub fn register_styles(cx: &mut Context) {
    // Distortion
    cx.add_stylesheet(include_str!("../css/overdrive.css")).unwrap();
    cx.add_stylesheet(include_str!("../css/fuzz.css")).unwrap();

    // Reverb
    cx.add_stylesheet(include_str!("../css/studio-reverb.css")).unwrap();
}

