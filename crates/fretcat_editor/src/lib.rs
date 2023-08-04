mod effects;
mod components;
mod keymap;

use std::sync::Arc;

use fretcat_effects::{AtomicRefCell, Chain, Overdrive, ChainCommand};

use keymap::make_keymap;
use nih_plug_vizia::{ViziaState, create_vizia_editor, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug::prelude::*;

use components::*;

pub type EditorState = ViziaState;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[allow(unused_parens)]
pub type InitFlags = (Arc<AtomicRefCell<Chain>>);

#[derive(Lens)]
struct EditorData {
    chain: Arc<AtomicRefCell<Chain>>,
}

impl Model for EditorData {}

pub fn create(
    #[allow(unused_parens)] (chain): InitFlags,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        EditorData {
            chain: chain.clone(),
        }
        .build(cx);
        make_keymap().build(cx);

        cx.add_stylesheet(include_str!("../css/editor.css"))
            .unwrap();
        cx.add_font_mem(include_bytes!("../res/SymbolsNerdFontMono-Regular.ttf"));

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Sidebar::new(cx);
            })
            .class("sidebar-wrapper");
            VStack::new(cx, |cx| {
                EffectList::new(cx);
            })
            .class("list-wrapper");
        })
        .class("main");

        card_system_init(cx);
    })
}
