use fretcat_effects::chain::ChainHandle;
use nih_plug_vizia::vizia::prelude::*;

pub fn effect_view(cx: &mut Context) {
    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
        Binding::new(cx, ChainHandle::effects, |cx, effects| {
            let effects = effects.get(cx);

            for handle in effects {
                let effect = handle.as_any();

                let mut height = 0.0;
                VStack::new(cx, |cx| {
                })
                .width(Percentage(100.0))
                .height(Pixels(height));
            }
        });
    }).class("effect-view");
}
