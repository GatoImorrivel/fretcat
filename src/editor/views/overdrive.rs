use nih_plug_vizia::vizia::prelude::*;

use crate::effects::{EffectHandle, overdrive::Overdrive};

#[derive(Debug, Clone, Copy, Lens)]
struct OverdriveData {
    pub(crate) gain: f32,
    pub(crate) volume: f32,
    handle: EffectHandle,
}

enum Message {
    Gain(f32),
    Volume(f32),
}

impl View for OverdriveData {
    fn element(&self) -> Option<&'static str> {
        Some("overdrive")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            Message::Gain(val) => {
                self.gain = *val;
                self.handle.as_mut_any().downcast_mut::<Overdrive>().unwrap().gain = *val;
            },
            Message::Volume(val) => {
                self.volume = *val;
                self.handle.as_mut_any().downcast_mut::<Overdrive>().unwrap().volume = *val;
            },
        });
    }
}

pub fn overdrive(cx: &mut Context, handle: EffectHandle) {
    let overdrive = handle.as_any().downcast_ref::<Overdrive>().unwrap();

    OverdriveData {
        gain: overdrive.gain,
        volume: overdrive.volume,
        handle: handle,
    }
    .build(cx, |cx| {
        cx.add_stylesheet(include_str!("./overdrive.css")).unwrap();
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, OverdriveData::gain, false)
                    .on_changing(|cx, val| cx.emit(Message::Gain(val)));
            });
            VStack::new(cx, |cx| {
                Knob::new(cx, 1.0, OverdriveData::volume, false)
                    .on_changing(|cx, val| cx.emit(Message::Volume(val)));
            });
        }).class("overdrive");
    });
}
