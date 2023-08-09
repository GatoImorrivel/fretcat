mod card;
mod effect_list;
mod effect_handle;
mod sidebar;

use nih_plug_vizia::vizia::prelude::Data;
use strum::{IntoEnumIterator, EnumIter};

pub use effect_list::{EffectList, EffectListEvent};
pub use sidebar::Sidebar;
pub use card::*;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq, Data)]
pub enum EffectKind {
    Distortion,
    Delay,
    Dynamics,
    Reverb,
    Echo,
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