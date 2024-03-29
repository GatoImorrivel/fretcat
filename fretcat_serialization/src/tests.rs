use fretcat_effects::Chain;

use crate::Preset;

#[test]
fn test_preset_serialization() {
    let chain = Chain::default();
    let mut preset_original = Preset::from(&chain);
    preset_original.set_name("Test");
    preset_original.save().unwrap();

    let preset_loaded = Preset::load("Test").unwrap();
    println!("{:#?}", preset_loaded);
}

#[test]
fn already_exist() {
    let mut preset = Preset::default();

    preset.set_name("asjfklsjlflsjklsjlksjflksjflkaafsawa");

    let e = preset.already_exists();
    assert_eq!(e, false);
}