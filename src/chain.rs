#[derive(Debug, Clone, Copy)]
pub struct Effect {

}

pub struct EffectChain {
    loaded_effects: Vec<Effect>
}

impl EffectChain {
    pub fn iter(&self) -> std::slice::Iter<'_, Effect> {
        self.loaded_effects.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Effect> {
        self.loaded_effects.iter_mut()
    }
}

impl<'a> IntoIterator for &'a EffectChain {
    type Item = &'a Effect;
    type IntoIter = std::slice::Iter<'a, Effect>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            loaded_effects: vec![]
        }
    }
}