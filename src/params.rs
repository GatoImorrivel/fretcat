use std::sync::{Arc, RwLock};
use nih_plug::{prelude::Params, params::persist};
use nih_plug_iced::IcedState;

use crate::{editor, effects::chain::EffectChain};

#[derive(Params)]
pub struct FretCatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,

    #[persist = "chain-state"]
    pub(crate) chain_state: RwLock<EffectChain>
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            chain_state: RwLock::new(EffectChain::default())
        }
    }
}