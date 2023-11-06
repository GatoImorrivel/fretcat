use std::collections::HashMap;

use nih_plug::vizia::{
    image::{load_from_memory_with_format, DynamicImage, ImageFormat},
    prelude::*,
};
use strum::{EnumIter, IntoEnumIterator};

use crate::systems::*;

pub const EDITOR_WIDTH: u32 = 1260;
pub const EDITOR_HEIGHT: u32 = 848;

pub const MAIN_COLOR: Color = Color::rgb(48, 47, 47);

lazy_static::lazy_static! {
    pub static ref EFFECT_CARDS: HashMap<EffectKind, Vec<Card>> = {
        let mut hashmap: HashMap<EffectKind, Vec<Card>> = HashMap::new();
        hashmap.insert(EffectKind::Distortion, vec![
            OVERDRIVE_CARD,
            FUZZ_CARD,
            DISTORTION_CARD
        ]);

        hashmap.insert(EffectKind::Delay, vec![]);
        hashmap.insert(EffectKind::Dynamics, vec![]);
        hashmap.insert(EffectKind::Reverb, vec![
            REVERB_CARD
        ]);

        hashmap
    };

}
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq, Data)]
pub enum EffectKind {
    Distortion,
    Delay,
    Dynamics,
    Reverb,
}

impl EffectKind {
    pub fn variants() -> Vec<Self> {
        EffectKind::iter().fold(vec![], |mut acc, kind| {
            acc.push(kind);
            acc
        })
    }
}

impl ToString for EffectKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn register_styles(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/editor.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/effect-list.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/effect-handle.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/sidebar.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/preset-control.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/audio-slider.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/cards.css")).unwrap();
    cx.add_stylesheet(include_str!("../css/message-system.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/card-list.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/mono-control.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/labeled-knob.css"))
        .unwrap();
}

#[inline]
pub fn darken(color: &Color, factor: f64) -> Color {
    let factor = factor.max(0.0).min(1.0);

    let darkened_red = (color.r() as f64 * factor) as u8;
    let darkened_green = (color.g() as f64 * factor) as u8;
    let darkened_blue = (color.b() as f64 * factor) as u8;

    Color::rgba(darkened_red, darkened_green, darkened_blue, color.a())
}

#[inline]
pub fn normalize(
    value: f32,
    min_input: f32,
    max_input: f32,
    min_output: f32,
    max_output: f32,
) -> f32 {
    let clamped_value = value.max(min_input).min(max_input);

    let input_range = max_input - min_input;
    let output_range = max_output - min_output;
    let normalized = (clamped_value - min_input) * output_range / input_range + min_output;

    normalized
}
