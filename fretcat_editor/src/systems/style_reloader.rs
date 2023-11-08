use super::super::hot_lib;

use nih_plug::{vizia::prelude::*, nih_log};

pub struct StyleReloader;

enum StyleReloaderEvent {
    Reload,
}

impl StyleReloader {
    pub fn new(cx: &mut Context) {
        Self {}.build(cx);
    }
}

impl Model for StyleReloader {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            StyleReloaderEvent::Reload => {
                nih_log!("bolas");
                if hot_lib::was_updated() {
                    cx.clear_styles();
                    cx.add_stylesheet(hot_lib::fretcat_styles()).unwrap();
                }
            }
        });
    }
}
