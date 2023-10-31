use fretcat_effects::{
    effects::{Mono, MonoState, PreFX},
    ChainData,
};
use nih_plug::vizia::prelude::*;

enum MonoMessage {
    ChangeState(MonoState),
}

#[derive(Debug, Clone, Copy, Default, Lens)]
pub struct MonoControl {
    mono_state: MonoState,
}

impl MonoControl {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self::default().build(cx, |cx| {
            VStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |ex| ex.emit(MonoMessage::ChangeState(MonoState::Left)),
                    |cx| Label::new(cx, "L"),
                )
                .class("left-btn")
                .toggle_class(
                    "selected-state",
                    Self::mono_state.map(|state| *state == MonoState::Left),
                );
                Button::new(
                    cx,
                    |ex| ex.emit(MonoMessage::ChangeState(MonoState::Off)),
                    |cx| Label::new(cx, ""),
                )
                .class("off-btn")
                .toggle_class(
                    "selected-state",
                    Self::mono_state.map(|state| *state == MonoState::Off),
                );
                Button::new(
                    cx,
                    |ex| ex.emit(MonoMessage::ChangeState(MonoState::Right)),
                    |cx| Label::new(cx, "R"),
                )
                .class("right-btn")
                .toggle_class(
                    "selected-state",
                    Self::mono_state.map(|state| *state == MonoState::Right),
                );
            });
        })
    }
}

impl View for MonoControl {
    fn element(&self) -> Option<&'static str> {
        Some("mono-control")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            MonoMessage::ChangeState(state) => {
                self.mono_state = *state;
                let chain = ChainData::as_mut_ex(cx);
                let mono = chain.get_pre_fx::<Mono>(&PreFX("mono")).unwrap();

                mono.set_state(*state);
            }
        });
    }
}
