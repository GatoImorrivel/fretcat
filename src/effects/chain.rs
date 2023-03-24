use std::{sync::{Arc, RwLock}, slice::{Iter, IterMut}, vec::IntoIter};

use nih_plug::params::persist::PersistentField;
use serde::{Deserialize, Serialize};

use crate::effects::{EffectState, Overdrive};

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectChain {
    chain: RwLock<Vec<EffectState>>,
}

impl EffectChain {
    pub fn add(&mut self, effect: EffectState) {
        self.chain
            .get_mut()
            .expect("Poisoned lock on add")
            .push(effect);
    }

    pub fn iter_mut(&mut self) -> IterMut<EffectState> {
        self.chain.get_mut().unwrap().iter_mut()
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            chain: RwLock::new(vec![EffectState::Overdrive(Overdrive::default()); 2]),
        }
    }
}

impl<'a> PersistentField<'a, EffectChain> for Arc<EffectChain> {
    fn set(&self, new_value: EffectChain) {
        let mut chain = self
            .chain
            .write()
            .expect("Poisoned write on set Effect chain");
        chain.clear();
        *chain = new_value.chain.into_inner().unwrap();
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&EffectChain) -> R,
    {
        f(self)
    }
}
