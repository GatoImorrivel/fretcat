mod components_tab;
mod presets_tab;

use nih_plug_vizia::vizia::prelude::*;

use self::components_tab::components_tab;
use self::presets_tab::presets_tab;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarTab {
    Preset,
    Component
}

impl Data for SidebarTab {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Lens, Clone)]
pub struct SidebarData {
    pub(crate) current_tab: SidebarTab
}

pub enum SidebarMessage {
    ChangeTab(SidebarTab)
}

impl Model for SidebarData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            SidebarMessage::ChangeTab(tab) => {
                self.current_tab = *tab;
            }
        });
    }
}


pub fn sidebar(cx: &mut Context) {
    cx.add_stylesheet(include_str!("../css/sidebar.css"))
        .unwrap();

    VStack::new(cx, |cx| {
        Binding::new(cx, SidebarData::current_tab, |cx, bind| {
            let tab = bind.get(cx);

            match tab {
                SidebarTab::Component => {
                    components_tab(cx);
                },
                SidebarTab::Preset => {
                    presets_tab(cx);
                }
            }
        });
    })
    .class("sidebar");
}