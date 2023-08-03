use strum::{EnumIter, IntoEnumIterator};

mod chain;
mod effect;

mod fuzz;
mod overdrive;

pub use atomic_refcell::*;

pub use fuzz::Fuzz;
pub use overdrive::Overdrive;
pub use chain::Chain;
pub use effect::{Effect, AudioEffect};
pub use dyn_clone;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Hash, Eq)]
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