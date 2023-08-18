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