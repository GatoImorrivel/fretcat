use nih_plug_vizia::vizia::prelude::Data;
use strum::{EnumIter, IntoEnumIterator};

pub mod chain;
pub mod effect;

pub mod fuzz;
pub mod overdrive;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
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

impl Data for EffectKind {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}