mod components;

use std::sync::{atomic::Ordering, Arc};

use fretcat_effects::{ChainData, ChainHandle};

use fretcat_common::nih_plug::prelude::*;
use fretcat_common::vizia::prelude::*;
use fretcat_common::{create_vizia_editor, ViziaState, ViziaTheming};

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
    pub bolas: String,
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
                    .store(fretcat_common::nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
            }
            EditorEvent::SetInGain(val) => {
                self.in_gain
                    .store(fretcat_common::nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
            }
            EditorEvent::SetOutGain(val) => {
                self.out_gain
                    .store(fretcat_common::nih_plug::util::db_to_gain(*val), Ordering::Relaxed);
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

        editor_data.clone().build(cx);
        fretcat_effects::register_fonts(cx);
        fretcat_effects::register_images(cx);
        fretcat_effects::register_styles(cx);
        register_styles(cx);

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
    cx.add_stylesheet(include_str!("../css/channel-slider.css"))
        .unwrap();
    cx.add_stylesheet(include_str!("../css/cards.css"))
        .unwrap();

}