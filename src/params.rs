use std::sync::Arc;

use fretcat_editor::EditorState;
use nih_plug::prelude::Params;

#[derive(Params, Debug)]
pub struct FretcatParams {
    #[persist = "editor-state"]
    pub(crate) editor_state: Arc<EditorState>,
}

impl Default for FretcatParams {
    fn default() -> Self {
        Self {
            editor_state: fretcat_editor::default_state(),
        }
    }
}
