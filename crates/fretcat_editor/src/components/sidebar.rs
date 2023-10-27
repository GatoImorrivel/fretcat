use fretcat_effects::{
    effects::{Gain, PostFX, PreFX},
    ChainData,
};
use nih_plug::vizia::prelude::*;

use crate::common::EffectKind;

use super::audio_slider::AudioSlider;

const KIND_PER_ROW: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum SidebarTab {
    Effect,
    Preset,
}

#[derive(Lens)]
pub struct Sidebar {
    pub current_tab: SidebarTab,
    pub selected_kind: EffectKind,
}

pub enum SidebarMessage {
    ChangeTab(SidebarTab),
}

impl Sidebar {
    pub fn new(cx: &mut Context, current_tab: SidebarTab) -> Handle<Self> {
        Self {
            current_tab,
            selected_kind: EffectKind::Distortion,
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Button::new(
                        cx,
                        |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Effect)),
                        |cx| Label::new(cx, "󰡀"),
                    )
                    .class("tab-btn")
                    .toggle_class("tab-selected-btn", Self::current_tab.map(|tab| *tab == SidebarTab::Effect));

                    Button::new(
                        cx,
                        |ex| ex.emit(SidebarMessage::ChangeTab(SidebarTab::Preset)),
                        |cx| Label::new(cx, ""),
                    )
                    .class("tab-btn")
                    .toggle_class("tab-selected-btn", Self::current_tab.map(|tab| *tab == SidebarTab::Preset));
                })
                .class("sidebar-buttons-wrapper").height(Percentage(15.0));

                VStack::new(cx, |cx| {
                    AudioSlider::new(
                        cx,
                        200.0,
                        ChainData::chain.map(|chain| chain.in_avg_amplitude),
                        |ex, val| {
                            let chain = ChainData::as_mut_ex(ex);
                            let prefx = chain
                                .get_pre_fx::<Gain>(&PreFX("in_gain"))
                                .expect("No in gain");
                            let gain = if val > -60.0 { val } else { 0.0 };
                            prefx.gain_in_db = gain;
                        },
                    );
                    AudioSlider::new(
                        cx,
                        200.0,
                        ChainData::chain.map(|chain| chain.out_avg_amplitude),
                        |ex, val| {
                            let chain = ChainData::as_mut_ex(ex);
                            let postfx = chain
                                .get_post_fx::<Gain>(&PostFX("out_gain"))
                                .expect("No out gain");
                            let gain = if val > -60.0 { val } else { 0.0 };
                            postfx.gain_in_db = gain;
                        },
                    );
                })
                .child_space(Stretch(1.0))
                .row_between(Stretch(1.0))
                .height(Percentage(85.0))
                .width(Stretch(1.0));
            })
            .class("bar");
        })
    }
}

impl View for Sidebar {
    fn element(&self) -> Option<&'static str> {
        Some("sidebar")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            SidebarMessage::ChangeTab(tab) => {
                self.current_tab = *tab;
            }
        });
    }
}
