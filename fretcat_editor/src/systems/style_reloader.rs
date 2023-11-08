use super::super::hot_lib;

use nih_plug::vizia::prelude::*;

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
            ApplicationEvent::NewFrame => {
                if hot_lib::was_updated() {
                    cx.clear_styles();
                    cx.emit(StyleReloaderEvent::Reload);
                }
            }
        });

        event.map(|event, _| match event {
            StyleReloaderEvent::Reload => {
                cx.add_stylesheet(hot_lib::fretcat_styles()).unwrap();
                cx.needs_restyle();
                cx.needs_relayout();
                cx.needs_redraw();
            }
        });
    }
}
