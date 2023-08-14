use crate::AudioEffect;

use serde::{Deserialize, Serialize, ser::SerializeStruct};

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
}

impl Serialize for Preset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Preset", 2)?;
        state.serialize_field("name", &self.name)?;
        
    }
}
