#[macro_export]
macro_rules! create_effects {
    ($( $effect:ident { $( $field:ident : $type:ty ),* } ),*) => {
        #[derive(Clone, Copy, Debug)]
        enum EffectState {
            $(
                $effect { $( $field : $type ),* }
            ),*
        }
    };
}