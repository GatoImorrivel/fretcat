use fretcat::fretcat_plugin::Fretcat;
use nih_plug::prelude::*;

fn main() {
    nih_export_standalone::<Fretcat>();
}
