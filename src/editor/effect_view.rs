use nih_plug_vizia::vizia::prelude::*;

use crate::{chain::ChainHandle, effects::Effect};

pub fn effect_view(
    cx: &mut Context,
) -> Handle<'_, VirtualList>
{
    VirtualList::new(cx, ChainHandle::effects, 200.0, |cx, i, effect| {
        Label::new(cx, "Bolas")
    })
}
