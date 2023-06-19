use nih_plug_vizia::vizia::{
    prelude::{Context, Handle, StyleModifiers, Units::Percentage, Lens},
    views::{Knob, KnobMode, TickKnob}
};

pub fn tick_knob<L: Lens<Target = f32>>(cx: &mut Context, lens: L) -> Handle<'_, Knob<L>> {
    Knob::custom(cx, 0.5, lens, move |cx, lens| {
        TickKnob::new(
            cx,
            Percentage(100.0),
            Percentage(20.0),
            Percentage(50.0),
            300.0,
            KnobMode::Continuous,
        )
        .value(lens)
        .class("track")
    })
}
