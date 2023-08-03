use fretcat::fretcat_plugin::Fretcat;
use fretcat::nih_plug::prelude::*;

fn main() {
    nih_export_standalone::<Fretcat>();
}
