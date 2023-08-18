mod mapper;

use std::{path::Path, fs::{File, self}, io::Write};

use fretcat_effects::{Chain, ChainHandle};
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

impl Preset {
    pub fn save(&self) {
        let home = home::home_dir().unwrap();
        let formatted = format!("{}/Documents/Fretcat/{}.json", home.display(), self.name);
        let preset_path = Path::new(&formatted);
        fs::create_dir_all(format!("{}/Documents/Fretcat", home.display())).unwrap();

        let mut file = File::create(preset_path).unwrap();
        let json = serde_json::to_string_pretty(self).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }
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

impl From<ChainHandle> for Preset {
    fn from(value: ChainHandle) -> Self {
        let mut me = Self::default();

        let mappers = value.borrow().effects.iter().map(|e| {
            Mapper::try_from(value.borrow().query(e).unwrap().clone()).unwrap()
        }).collect();

        me.effects = mappers;

        me
    }
}