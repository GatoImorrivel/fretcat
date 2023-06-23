use std::{fmt::format, ops::Range, any::TypeId};

use nih_plug::{nih_log, nih_error};
use nih_plug_vizia::vizia::prelude::*;

use crate::{
    chain::ChainHandle,
    effects::{Effect, EffectHandle, overdrive::Overdrive},
};

use super::overdrive::overdrive;

pub fn effect_view(cx: &mut Context) {
    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
        Binding::new(cx, ChainHandle::effects, |cx, effects| {
            let effects = effects.get(cx);

            for handle in effects {
                let effect = handle.as_any();

                let mut height = 0.0;
                VStack::new(cx, |cx| {
                    match effect.type_id() {
                        id if id == TypeId::of::<Overdrive>() => {
                            overdrive(cx, handle);
                            height = 200.0;
                        },

                        _ => nih_error!("INVALID DOWNCAST")
                    }
                })
                .width(Percentage(100.0))
                .height(Pixels(height));
            }
        });
    })
    .width(Percentage(79.0))
    .height(Percentage(100.0));
}
