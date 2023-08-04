use std::{marker::PhantomData, sync::Arc};

use editor_derive::Control;
use fretcat_effects::{AtomicRefCell, AudioEffect, Chain, Effect};

mod overdrive;

#[derive(Debug, Clone)]
pub struct EffectHandle<T: AudioEffect> {
    effect: Effect,
    chain: Arc<AtomicRefCell<Chain>>,
    p: PhantomData<T>,
}
