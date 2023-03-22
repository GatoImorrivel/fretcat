use std::{sync::{Arc, RwLock}, slice::Iter, vec::IntoIter};

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
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            chain: RwLock::new(vec![EffectState::Overdrive(Overdrive::default())]),
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
        chain.append(
            &mut new_value
                .chain
                .read()
                .expect("Poisoned read of new value")
                .clone(),
        );
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&EffectChain) -> R,
    {
        f(self)
    }
}
