mod mapper;
#[cfg(test)]
mod tests;

use std::{fs, path::{Path, PathBuf}, sync::Arc, io};

use fretcat_effects::{effects::AudioEffect, Chain};
use lazy_static::lazy_static;
use mapper::Mapper;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, Display, IntoEnumIterator};
use nih_plug::vizia::prelude::Data;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, EnumIter, Display, PartialEq, Eq, Data)]
pub enum PresetCategory {
    #[default]
    User,
    Ambient,
    Rock,
    Jazzy
}

impl PresetCategory {
    pub fn variants() -> Vec<Self> {
        PresetCategory::iter().fold(vec![], |mut acc, kind| {
            acc.push(kind);
            acc
        })
    }
}

lazy_static! {
    pub static ref PRESET_CATEOGORY_LIST: Vec<String> = PresetCategory::iter().map(|category| category.to_string()).collect::<Vec<_>>();
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Preset {
    name: String,
    category: PresetCategory,
    effects: Vec<Mapper>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShallowPreset {
    name: String,
    category: PresetCategory
}

impl ShallowPreset {
    pub fn load(self) -> Preset {
        self.into()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_category(&self) -> PresetCategory {
        self.category
    }
}

impl From<Preset> for ShallowPreset {
    fn from(value: Preset) -> Self {
        Self {
            name: value.get_name().to_owned(),
            category: value.get_category()
        }
    }
}

impl Into<Preset> for ShallowPreset {
    fn into(self) -> Preset {
        let mut p = Preset::default();
        p.set_name(self.name.to_owned());
        p.category = self.category;
        p.load_effects();
        p
    }
}

impl Preset {
    pub fn fetch_presets_shallow() -> Vec<ShallowPreset> {
        let paths = fs::read_dir(Self::get_preset_dir()).unwrap();
        let mut presets = vec![];
        for path in paths {
            if let Ok(path) = path {
                let json = fs::read_to_string(path.path()).unwrap();
                let preset = serde_json::from_str::<Preset>(&json);
                if let Ok(mut preset) = preset {
                    preset.effects.clear();
                    presets.push(ShallowPreset::from(preset));
                }
            }
        }
        presets
    }

    pub fn get_category(&self) -> PresetCategory {
        self.category
    }

    pub fn load_effects(&mut self) {
        let json = fs::read_to_string(self.get_preset_path()).unwrap();
        let preset = serde_json::from_str::<Preset>(&json).unwrap();

        self.set_mappers(preset.cloned_mappers());
    }

    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().to_owned();
    }

    pub fn get_name(&self) -> &str {
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

    pub fn set_mappers(&mut self, mappers: Vec<Mapper>) {
        self.effects = mappers;
    }

    pub fn cloned_mappers(&self) -> Vec<Mapper> {
        self.effects.clone()
    }

    pub fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();

        fs::write(self.get_preset_path(), json)
    }

    pub fn delete(&self) -> io::Result<()> {
        fs::remove_file(self.get_preset_path())
    }

    pub fn overwrite(&self) -> io::Result<()> {
        self.delete()?;
        self.save()
    }

    pub fn get_preset_dir() -> PathBuf {
        let home = home::home_dir().unwrap();
        fs::create_dir_all(format!("{}/Documents/Fretcat", home.display())).unwrap();
        let formatted = format!("{}/Documents/Fretcat", home.display());
        Path::new(&formatted).to_owned()
    }

    pub fn get_preset_path(&self) -> PathBuf {
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
            .map(|e| Mapper::try_from(e.handle()).unwrap())
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
            .map(|e| Mapper::try_from(e.handle()).unwrap())
            .collect();

        me.effects = mappers;

        me
    }
}

impl Into<Vec<Arc<dyn AudioEffect>>> for Preset {
    fn into(self) -> Vec<Arc<dyn AudioEffect>> {
        self.effects
            .into_iter()
            .fold(Vec::<Arc<dyn AudioEffect>>::new(), |mut acc, mapper| {
                let effect: Arc<dyn AudioEffect> = mapper.try_into().unwrap();
                acc.push(effect);
                acc
            })
    }
}