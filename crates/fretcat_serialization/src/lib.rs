mod mapper;
#[cfg(test)]
mod tests;

use std::{fs, path::Path};

use fretcat_effects::{AudioEffect, Chain, ChainHandle};
use mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum PresetCategory {
    #[default]
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    name: String,
    category: PresetCategory,
    effects: Vec<Mapper>,
}

impl Preset {
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().to_owned();
    }

    pub fn save(&self) {
        let home = home::home_dir().unwrap();
        let formatted = format!("{}/Documents/Fretcat/{}.json", home.display(), self.name);
        let preset_path = Path::new(&formatted);
        fs::create_dir_all(format!("{}/Documents/Fretcat", home.display())).unwrap();

        let json = serde_json::to_string_pretty(self).unwrap();

        fs::write(preset_path, json).unwrap();
    }

    pub fn load<S: AsRef<str>>(preset_name: S) -> Option<Self> {
        let home = home::home_dir().unwrap();
        let formatted = format!(
            "{}/Documents/Fretcat/{}.json",
            home.display(),
            preset_name.as_ref()
        );
        let preset_path = Path::new(&formatted);

        let json = match fs::read_to_string(preset_path) {
            Ok(json) => json,
            Err(_) => return None,
        };

        match serde_json::from_str::<Self>(&json) {
            Ok(preset) => Some(preset),
            Err(_) => None,
        }
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "Untitled".to_owned(),
            category: PresetCategory::default(),
            effects: vec![],
        }
    }
}

impl From<ChainHandle> for Preset {
    fn from(value: ChainHandle) -> Self {
        let mut me = Self::default();

        let mappers = value
            .borrow()
            .effects
            .iter()
            .map(|e| Mapper::try_from(value.borrow().query(e).unwrap().clone()).unwrap())
            .collect();

        me.effects = mappers;

        me
    }
}

impl From<&Chain> for Preset {
    fn from(value: &Chain) -> Self {
        let mut me = Self::default();

        let mappers = value
            .effects
            .iter()
            .map(|e| Mapper::try_from(value.query(e).unwrap().clone()).unwrap())
            .collect();

        me.effects = mappers;

        me
    }
}

impl Into<Vec<Box<dyn AudioEffect>>> for Preset {
    fn into(self) -> Vec<Box<dyn AudioEffect>> {
        self.effects
            .into_iter()
            .fold(Vec::<Box<dyn AudioEffect>>::new(), |mut acc, mapper| {
                let effect: Box<dyn AudioEffect> = mapper.try_into().unwrap();
                acc.push(effect);
                acc
            })
    }
}
