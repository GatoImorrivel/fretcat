#[macro_export]
macro_rules! create_messages {
    ($( $effect:ident { $( $field:ident : $type:ty ),* } ),*) => {
        enum EffectState {
            $(
                $effect { $( $field : $type ),* }
            ),*
        }
    };
}