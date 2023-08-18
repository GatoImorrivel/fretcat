mod mapper;

use fretcat_effects::Chain;
use mapper::Mapper;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum PresetCategory {
    #[default]
    User
}

#[derive(Debug, Serialize)]
pub struct Preset {
    name: String,
    category: PresetCategory,
    effects: Vec<Mapper> 
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Untitled".to_owned(),
            category: PresetCategory::default(),
            effects: vec![]
        }
    }
}

impl From<Chain> for Preset {
    fn from(value: Chain) -> Self {
        let me = Self::default();

        me
    }
}