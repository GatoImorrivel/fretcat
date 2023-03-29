use std::{sync::{Arc, RwLock}, slice::{Iter, IterMut}, vec::IntoIter};

use nih_plug::params::persist::PersistentField;
use serde::{Deserialize, Serialize};

use crate::effects::{Effects, Overdrive};

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectChain {
    chain: Vec<Effects>,
}

impl EffectChain {
    pub fn add(&mut self, effect: Effects) {
        self.chain.push(effect);
    }

    pub fn iter_mut(&mut self) -> IterMut<Effects> {
        self.iter_mut()
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            chain: vec![Overdrive::default().into(); 2],
        }
    }
}