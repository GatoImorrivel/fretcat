use std::sync::Arc;

use nih_plug::{prelude::{Params, FloatParam, FloatRange}, params::persist};
use nih_plug_vizia::ViziaState;

#[derive(Params, Debug)]
pub struct FretcatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<ViziaState>,
}

impl Default for FretcatParams {
    fn default() -> Self {
        Self {
            editor_state: fretcat_editor::default_state(),
        }
    }
}
