macro_rules! effects {
    ($($effect:ident),*) => {
        #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
        pub enum Effects {
            $(
                $effect($effect),
            )*
        }

        $(
            impl From<$effect> for Effects {
                fn from(effect: $effect) -> Self {
                    Effects::$effect(effect)
                }
            }

            impl From<Effects> for $effect {
                fn from(effects: Effects) -> $effect {
                    match effects {
                        Effects::$effect(effect) => effect,
                        _ => unreachable!(),
                    }
                }
            }
        )*
    };
}

macro_rules! effect_messages {
    ($($effect:ident),*) => {
        #[derive(Debug, Copy, Clone)]
        pub enum EffectMessages {
            $(
                $effect($effect),
            )*
        }

        $(
            impl From<$effect> for EffectMessages {
                fn from(effect: $effect) -> Self {
                    EffectMessages::$effect(effect)
                }
            }
        )*
    };
}
