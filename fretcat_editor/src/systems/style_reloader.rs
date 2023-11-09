use std::sync::Mutex;

use super::super::hot_lib;

use hot_lib_reloader::LibReloadObserver;
use nih_plug::vizia::prelude::*;

lazy_static::lazy_static! {
    static ref HAS_RELOADED: Mutex<bool> = Mutex::new(false);
    pub static ref STYLES: Mutex<String> = Mutex::new(hot_lib::fretcat_styles().to_string());
}

pub struct StyleReloader;

enum StyleReloaderEvent {
    Reload,
}

impl StyleReloader {
    pub fn new(cx: &mut Context) {
        std::thread::spawn(|| {
            let observer: LibReloadObserver = hot_lib::subscribe();
            loop {
                observer.wait_for_reload();
                *STYLES.lock().unwrap() = hot_lib::fretcat_styles().to_string();
                *HAS_RELOADED.lock().unwrap() = true;
            }
        });
        Self {}.build(cx);
    }
}

impl Model for StyleReloader {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            ApplicationEvent::NewFrame => {
                if *HAS_RELOADED.lock().unwrap() {
                    cx.clear_styles();
                    cx.emit(StyleReloaderEvent::Reload);
                }
            }
        });

        event.map(|event, _| match event {
            StyleReloaderEvent::Reload => {
                cx.add_stylesheet(CSS::from_string(STYLES.lock().unwrap().as_str())).unwrap();
                cx.reload_styles().unwrap();
            }
        });
    }
}
