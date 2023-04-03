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
