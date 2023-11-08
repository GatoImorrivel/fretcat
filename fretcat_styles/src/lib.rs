#[no_mangle]
pub fn fretcat_styles() -> &'static str {
    include_str!("../fretcat-styles.css")
}