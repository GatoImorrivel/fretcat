mod card;
mod preset_control;
mod effect_list;
mod effect_handle;
mod sidebar;
mod audio_slider;
mod message_system;

use nih_plug::vizia::prelude::*;
use strum::{IntoEnumIterator, EnumIter};

pub use effect_list::{EffectList, EffectListEvent};
pub use sidebar::Sidebar;
pub use preset_control::PresetControl;
pub use message_system::*;
pub use card::*;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq, Data)]
pub enum EffectKind {
    Distortion,
    Delay,
    Dynamics,
    Reverb
}

impl EffectKind {
    pub fn variants() -> Vec<Self> {
        EffectKind::iter().fold(vec![], |mut acc, kind| {
            acc.push(kind);
            acc
        })
    }
}

impl ToString for EffectKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
