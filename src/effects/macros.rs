macro_rules! messages {
    ($($msg:ident),*) => {
        #[derive(Copy, Clone, Debug)]
        pub enum EffectMessage {
            $(
                $msg($msg),
            )*
        }

        $(
            impl From<$msg> for EffectMessage {
                fn from(value: $msg) -> EffectMessage {
                    EffectMessage::$msg(value)
                }
            }
        )*
    };
}
