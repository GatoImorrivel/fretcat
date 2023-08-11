pub use nih_plug_vizia::vizia::prelude::*;

pub struct PresetControl {}

pub enum PresetMessage {
    Save,
}

impl PresetControl {
    pub fn new(cx: &mut Context) {
        Self {}.build(cx, |cx| {
            cx.add_stylesheet(include_str!("../../css/preset-control.css"))
                .unwrap();
            HStack::new(cx, |cx| {
                Label::new(cx, "untitled").class("preset-name");
                Button::new(
                    cx,
                    |ex| ex.emit(PresetMessage::Save),
                    |cx| Label::new(cx, "󰆓"),
                )
                .class("save-btn");
            });
        });
    }
}

impl View for PresetControl {
    fn element(&self) -> Option<&'static str> {
        Some("preset-control")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}
}
