macro_rules! ui_updates {
    ($($msg:ident),*) => {
        pub enum UIMessage {
            $(
                $msg($msg),
            )*
        }
    };
}
