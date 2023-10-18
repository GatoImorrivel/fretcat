mod components;

use std::sync::{atomic::Ordering, Arc};

use fretcat_effects::{ChainData, Chain};

use fretcat_serialization::Preset;
use nih_plug::prelude::*;
use nih_plug::vizia::prelude::*;
use nih_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use components::*;

pub type EditorState = ViziaState;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[allow(unused_parens)]
pub type InitFlags = (Arc<Chain>, EditorData);

#[derive(Debug, Clone, Lens, Default)]
pub struct EditorData {
    pub current_preset: Preset,
    pub original_preset: Preset
}

impl EditorData {
    pub fn restore_preset(&mut self) {
        self.current_preset = self.original_preset.clone();
    }
}

impl Model for EditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    }
}

pub fn create(
    #[allow(unused_parens)] (chain, editor_data): InitFlags,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        ChainData {
            chain: chain.clone(),
        }
        .build(cx);

        editor_data.clone().build(cx);
        fretcat_effects::register_fonts(cx);
        fretcat_effects::register_images(cx);
        fretcat_effects::register_styles(cx);
        register_styles(cx);

        CardSystem::init(cx);
        MessageSystem::init(cx);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Sidebar::new(cx);
            })
            .class("sidebar-wrapper");
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    PresetControl::new(cx);
                })
                .class("preset-control-wrapper");
                VStack::new(cx, |cx| {
                    EffectList::new(cx);
                    MessageSystem::view(cx);
                })
                .class("effect-list-wrapper");
            })
            .class("right-wrapper");
        })
        .class("main");

        CardSystem::view(cx);
    })
}

fn register_styles(cx: &mut Context) {
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
    cx.add_stylesheet(include_str!("../css/cards.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/message-system.css"))
        .unwrap();
}

pub fn darken(color: &Color, factor: f64) -> Color {
    let factor = factor.max(0.0).min(1.0);

    let darkened_red = (color.r() as f64 * factor) as u8;
    let darkened_green = (color.g() as f64 * factor) as u8;
    let darkened_blue = (color.b() as f64 * factor) as u8;

    Color::rgba(darkened_red, darkened_green, darkened_blue, color.a())
}