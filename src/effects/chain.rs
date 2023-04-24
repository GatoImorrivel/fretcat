use nih_plug::nih_log;

use super::{Effect, ui::EffectUI, EffectUpdate, OverdriveMessage, OverdriveEffect};

pub struct Chain {
    effects: Vec<Box<dyn Effect + Send + Sync>>,
}

impl Chain {
    pub fn process(&self, mut sample: f32) -> f32 {
        for effect in &self.effects {
            sample = effect.process(sample);
        }

        sample
    }

    pub fn update(&mut self, update: EffectUpdate) {
        let (id, message) = update.take();

        self.effects[id].update(message);
    }

    pub fn build_ui(&self) -> Vec<Box<dyn EffectUI + Send + Sync>> {
        let mut uis = vec![];
        for (id, effect) in self.effects.iter().enumerate() {
            uis.push(effect.ui(id));
        }

        uis
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            effects: vec![Box::new(OverdriveEffect::default())]
        }
    }
}