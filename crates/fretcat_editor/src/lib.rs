mod components;
mod keymap;

use std::sync::{atomic::Ordering, Arc};

use fretcat_effects::{ChainData, ChainHandle};

use keymap::make_keymap;
use nih_plug::prelude::*;
use nih_plug_vizia::vizia::image::imageops::FilterType;
use nih_plug_vizia::vizia::image::{load_from_memory_with_format, DynamicImage, ImageFormat};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use components::*;

pub type EditorState = ViziaState;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[allow(unused_parens)]
pub type InitFlags = (ChainHandle, EditorData);

#[derive(Debug, Clone, Lens, Default)]
pub struct EditorData {
    pub noise_gate: Arc<AtomicF32>,
    pub in_gain: Arc<AtomicF32>,
    pub out_gain: Arc<AtomicF32>,
}

pub enum EditorEvent {
    SetNoiseGate(f32),
    SetInGain(f32),
    SetOutGain(f32),
}

impl Model for EditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            EditorEvent::SetNoiseGate(val) => {
                self.noise_gate
                    .store(nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
            }
            EditorEvent::SetInGain(val) => {
                self.in_gain
                    .store(nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
            }
            EditorEvent::SetOutGain(val) => {
                self.out_gain
                    .store(nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
            }
        });
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
        make_keymap().build(cx);

        editor_data.clone().build(cx);
        register_fonts(cx);
        register_images(cx);

        cx.add_stylesheet(include_str!("../css/editor.css"))
            .unwrap();
        card_system_init(cx);

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
                })
                .class("effect-list-wrapper");
            })
            .class("right-wrapper");
        })
        .class("main");

        card_system_view(cx);
    })
}

fn register_fonts(cx: &mut Context) {
    cx.add_font_mem(include_bytes!(
        "../../assets/fonts/SymbolsNerdFontMono-Regular.ttf"
    ));
    cx.add_font_mem(include_bytes!("../../assets/fonts/Saturday.otf"));
}

fn register_images(cx: &mut Context) {
    let image = load_from_memory_with_format(
        include_bytes!("../../assets/images/drive-bg.png"),
        ImageFormat::Png,
    )
    .unwrap()
    .resize_to_fill(EDITOR_WIDTH / 2, EDITOR_HEIGHT / 2, FilterType::Lanczos3)
    .brighten(-50)
    .to_rgb8();

    cx.load_image(
        "drive-background",
        DynamicImage::ImageRgb8(image),
        ImageRetentionPolicy::Forever,
    )
    .build(cx);
}
