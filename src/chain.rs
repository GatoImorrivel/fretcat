use std::sync::RwLock;

use nih_plug::params::persist::PersistentField;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct EffectChain {
    chain: RwLock<Vec<EffectState>>,
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            chain: RwLock::new(vec![]),
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EffectState {
    Overdrive { gain: f32 },
}
