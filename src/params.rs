use std::sync::Arc;

use nih_plug::{params::persist, prelude::Params};
use nih_plug_iced::IcedState;

use crate::{effects::{field::VecField, Effects, Overdrive}, editor};

#[derive(Params)]
pub struct FretCatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<IcedState>,

    #[persist = "chain-state"]
    pub(crate) chain_state: VecField<'static, Effects>
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            chain_state: VecField::new(vec![Overdrive::default().into()])
        }
    }
}