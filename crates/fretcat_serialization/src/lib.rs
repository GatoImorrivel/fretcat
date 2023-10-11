mod mapper;
#[cfg(test)]
mod tests;

use std::{fs, path::{Path, PathBuf}, sync::Arc};

use fretcat_effects::{effects::AudioEffect, Chain};
use mapper::Mapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum PresetCategory {
    #[default]
    User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preset {
    name: String,
    category: PresetCategory,
    effects: Vec<Mapper>,
}

impl Preset {
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().to_owned();
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn already_exists(&self) -> bool {
        let paths = fs::read_dir(Self::get_preset_dir()).unwrap();
        for path in paths {
            if self.get_preset_path() == path.unwrap().path() {
                return true; 
            }
        }

        false
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();

        fs::write(self.get_preset_path(), json).unwrap();
    }

    pub fn get_preset_dir() -> PathBuf {
        let home = home::home_dir().unwrap();
        fs::create_dir_all(format!("{}/Documents/Fretcat", home.display())).unwrap();
        let formatted = format!("{}/Documents/Fretcat", home.display());
        Path::new(&formatted).to_owned()
    }

    pub fn get_preset_path(&self) -> PathBuf {
        let home = home::home_dir().unwrap();
        let formatted = format!("{}/{}.json", Self::get_preset_dir().display(), self.name);
        Path::new(&formatted).to_owned()
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

impl From<Arc<Chain>> for Preset {
    fn from(value: Arc<Chain>) -> Self {
        let mut me = Self::default();

        let mappers = value
            .effects
            .iter()
            .map(|e| Mapper::try_from(e.clone()).unwrap())
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
            .map(|e| Mapper::try_from(e.clone()).unwrap())
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
