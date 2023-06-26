pub mod editor;
mod sidebar;
mod top_bar;
mod effect_view;

use std::sync::Arc;
use nih_plug_vizia::ViziaState;

pub const EDITOR_WIDTH: u32 = 1260;
pub const EDITOR_HEIGHT: u32 = 848;

pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}