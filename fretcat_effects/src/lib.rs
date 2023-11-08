mod chain;
mod common;
mod components;
pub mod effects;

use std::sync::Arc;

use std::ops::{Deref, DerefMut};

use effects::AudioEffect;
use nih_plug::vizia::{
    image::{imageops, load_from_memory_with_format, DynamicImage, ImageFormat},
    prelude::*,
};

pub use chain::{Chain, ChainCommand, ChainData, NUM_CHANNELS};
pub use dyn_clone;

pub fn register_fonts(cx: &mut Context) {
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/SymbolsNerdFontMono-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/MajorMonoDisplay-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Montserrat.ttf"));
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

    let studio_reverb = studio_reverb
        .resize_to_fill(width * 2, height, imageops::FilterType::Nearest)
        .to_rgb8();

    cx.load_image(
        "reverb-background",
        DynamicImage::ImageRgb8(studio_reverb),
        ImageRetentionPolicy::Forever,
    );
}

#[derive(Debug, Clone)]
pub struct EffectHandle<T: AudioEffect> {
    handle: Arc<T>,
}

impl<T: AudioEffect> Deref for EffectHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.handle.as_ref()
    }
}

impl<T: AudioEffect> DerefMut for EffectHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: AudioEffect> From<Arc<T>> for EffectHandle<T> {
    fn from(value: Arc<T>) -> Self {
        EffectHandle::new(value)
    }
}

impl<T: AudioEffect> From<Arc<dyn AudioEffect>> for EffectHandle<T> {
    fn from(value: Arc<dyn AudioEffect>) -> Self {
        EffectHandle::new(value.into_any_arc().downcast::<T>().unwrap())
    }
}

impl<T: AudioEffect> EffectHandle<T> {
    pub fn new(handle: Arc<T>) -> Self {
        Self {
            handle: handle.clone()
        }
    }
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { Arc::as_ptr(&self.handle).cast_mut().as_mut().unwrap() }
    }
}
