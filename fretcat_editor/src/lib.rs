mod common;
mod components;
mod systems;
#[hot_lib_reloader::hot_module(dylib = "fretcat_styles", lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../","/target/debug"))]
mod hot_lib {
    hot_functions_from_file!("fretcat_styles/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}
use std::sync::{Arc, Mutex};

use common::{EDITOR_HEIGHT, EDITOR_WIDTH};
use fretcat_effects::{Chain, ChainCommand, ChainData};

use fretcat_serialization::{Preset, ShallowPreset};
use nih_plug::prelude::*;
use nih_plug::vizia::prelude::*;
use nih_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::components::*;
use crate::systems::*;

pub type EditorState = ViziaState;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[derive(Debug, Lens, Clone)]
pub struct EditorData {
    pub(crate) current_tab: SidebarTab,
    pub(crate) current_preset: Arc<Mutex<Preset>>,
}

pub enum EditorEvent {
    LoadPreset(Preset),
    LoadShallowPreset(ShallowPreset),
}

impl Model for EditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            SidebarMessage::ChangeTab(tab) => {
                self.current_tab = *tab;
            }
        });

        event.map(|event, _| match event {
            EditorEvent::LoadPreset(p) => {
                *self.current_preset.lock().unwrap() = p.clone();
                cx.emit(ChainCommand::Load(p.clone().into()));
            }
            EditorEvent::LoadShallowPreset(p) => {
                let p = p.clone().load();
                *self.current_preset.lock().unwrap() = p.clone();
                cx.emit(ChainCommand::Load(p.into()));
            }
        });
    }
}

pub fn create(
    chain: Arc<Chain>,
    preset: Arc<Mutex<Preset>>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::None, move |cx, _| {
        ChainData {
            chain: chain.clone(),
        }
        .build(cx);

        EditorData {
            current_tab: SidebarTab::Effect,
            current_preset: preset.clone(),
        }
        .build(cx);

        fretcat_effects::register_fonts(cx);

        cx.add_stylesheet(CSS::from_string(STYLES.lock().unwrap().as_str())).unwrap();
        StyleReloader::new(cx);

        CardSystem::init(cx);

        HStack::new(cx, |cx| {
            Sidebar::new(cx, EditorData::current_tab.get(cx)).width(Stretch(0.7));

            CardList::new(cx)
                .width(Stretch(3.5))
                .row_between(Percentage(2.0))
                .display(EditorData::current_tab.map(|tab| *tab == SidebarTab::Effect));
            PresetList::new(cx)
                .width(Stretch(3.5))
                .row_between(Percentage(2.0))
                .display(EditorData::current_tab.map(|tab| *tab == SidebarTab::Preset));

            VStack::new(cx, |cx| {
                PresetControl::new(cx).height(Stretch(1.0)).z_index(200);
                EffectList::new(cx, ChainData::chain).height(Stretch(6.0));
                MessageSystem::new(cx).top(Stretch(1.0));
            })
            .row_between(Percentage(3.0))
            .width(Stretch(10.0));
        })
        .class("main");

        CardSystem::view(cx);
    })
}
