use nih_plug_vizia::vizia::prelude::*;

pub fn sidebar(cx: &mut Context) -> Handle<'_, VStack> {
    VStack::new(cx, |cx| {
        Label::new(cx, "Bolas");
    })
}
