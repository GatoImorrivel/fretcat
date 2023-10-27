mod common;
mod components;
mod systems;

use std::sync::Arc;

use common::{register_styles, EDITOR_HEIGHT, EDITOR_WIDTH, EFFECT_CARDS, MAIN_COLOR};
use fretcat_effects::{Chain, ChainData};

use nih_plug::prelude::*;
use nih_plug::vizia::prelude::*;
use nih_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::components::*;
use crate::systems::*;

pub type EditorState = ViziaState;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

pub type InitFlags = Arc<Chain>;

#[derive(Debug, Lens, Clone)]
pub struct EditorData {
    pub(crate) current_tab: SidebarTab
}

impl Model for EditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            SidebarMessage::ChangeTab(tab) => {
                self.current_tab = *tab;
            }
        });
    }
}

pub fn create(chain: InitFlags, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        ChainData {
            chain: chain.clone(),
        }
        .build(cx);

        EditorData {
            current_tab: SidebarTab::Effect
        }.build(cx);

        fretcat_effects::register_fonts(cx);
        fretcat_effects::register_images(cx);
        fretcat_effects::register_styles(cx);
        register_styles(cx);

        CardSystem::init(cx);
        MessageSystem::init(cx);

        HStack::new(cx, |cx| {
            Sidebar::new(cx, EditorData::current_tab.get(cx)).width(Stretch(0.7))
                .background_color(MAIN_COLOR);

            CardList::new(cx).width(Stretch(3.5)).display(EditorData::current_tab.map(|tab| *tab == SidebarTab::Effect));
            PresetList::new(cx).width(Stretch(3.5)).display(EditorData::current_tab.map(|tab| *tab == SidebarTab::Preset));

            VStack::new(cx, |cx| {
                PresetControl::new(cx, Some(ChainData::chain)).height(Stretch(1.0)).z_index(200);
                EffectList::new(cx, ChainData::chain).height(Stretch(6.0));
            })
            .width(Stretch(10.0));
        })
        .class("main")
        .background_color(MAIN_COLOR);

        CardSystem::view(cx);
    })
}
