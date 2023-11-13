mod chain;
mod frame;
mod common;
mod components;
pub mod effects;

mod effect_handle;
pub mod prelude;

use nih_plug::vizia::prelude::*;

pub use chain::{Chain, ChainCommand, ChainData, NUM_CHANNELS};
pub use dyn_clone;

pub fn register_fonts(cx: &mut Context) {
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/SymbolsNerdFontMono-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/Marvel-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/Marvel-Italic.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/Marvel-BoldItalic.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/Marvel-Bold.ttf"
    ));
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/Sparkler-demo.ttf"
    ));
}