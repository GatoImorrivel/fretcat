use std::sync::{Arc, RwLock};
use nih_plug::{prelude::Params};
use nih_plug_iced::IcedState;

use crate::{editor, effects::{Effects, Overdrive}};

#[derive(Params)]
pub struct FretCatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,

    #[persist = "chain-state"]
    pub(crate) chain_state: Arc<RwLock<Vec<Effects>>>
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            chain_state: Arc::new(RwLock::new(vec![Effects::from(Overdrive::default()); 4]))
        }
    }
}