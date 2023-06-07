use std::sync::Arc;

use nih_plug::{params::persist, prelude::Params};
use nih_plug_vizia::ViziaState;

#[derive(Params, Debug)]
pub struct FretcatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<ViziaState>,
}

impl Default for FretcatParams {
    fn default() -> Self {
        Self {
            editor_state: crate::editor::default_state()
        }
    }
}
