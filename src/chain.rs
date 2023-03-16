#[derive(Debug, Clone, Copy)]
pub struct Effect {

}

pub struct EffectChain {
    loaded_effects: Vec<Effect>
}

impl Default for EffectChain {
    fn default() -> Self {
        Self {
            loaded_effects: vec![]
        }
    }
}