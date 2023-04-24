use std::sync::{Arc};
use crossbeam::atomic::AtomicCell;
use nih_plug::{prelude::Params, params::persist};
use nih_plug_iced::IcedState;

use crate::{editor};


#[derive(Params)]
pub struct FretCatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}