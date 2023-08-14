use std::path::Path;

use crate::AudioEffect;

use erased_serde::Serialize;

#[derive(Debug)]
pub struct Preset {
    name: String,
    effects: Vec<Box<dyn AudioEffect>>,
}

impl Preset {
    pub fn new(name: &str, effects: Vec<Box<dyn AudioEffect>>) -> Self {
        Self {
            name: name.to_owned(),
            effects,
        }
    }

    pub fn save(&self) {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/Documents/chain.json", home);
        let path = Path::new(&path);
        let mut json: String = "".to_owned();
    }
}

impl Serialize for Preset {
    fn erased_serialize(&self, v: &mut dyn erased_serde::Serializer) -> Result<(), erased_serde::Error> {
        let mut state = v.erased_serialize_struct("Preset", 2)?;
        state.serialize_field("name", &self.name)?;
        let serialized = self.effects.iter().fold(Vec::<String>::new(), |acc, e| {
            let str = e.erased_serialize(v);
            acc
        });
        state.serialize_field("effects", self.effects, S);
        state.end()
    }
}