mod params;

use fretcat_effects::chain::{Chain, ChainHandle};
pub use params::FretcatParams;

use nih_plug::{prelude::AtomicF32};
use nih_plug::{
    nih_export_vst3,
    prelude::{
        AsyncExecutor, AudioIOLayout, AuxiliaryBuffers, Buffer, BufferConfig, InitContext, Params,
        Plugin, ProcessContext, ProcessStatus, Vst3Plugin, Vst3SubCategory, Editor,
    },
};
use std::{num::NonZeroU32, sync::Arc, cell::Cell};

const NUM_INPUT_CHANNELS: u32 = 2;
const NUM_OUTPUT_CHANNELS: u32 = 2;

pub struct Fretcat {
    params: Arc<FretcatParams>,
    chain: Cell<Chain>,
    noise_gate: Arc<AtomicF32>,
}

impl Default for Fretcat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretcatParams::default()),
            chain: Cell::new(Chain::default()),
            noise_gate: Arc::new(0.0.into())
        }
    }
}

impl Plugin for Fretcat {
    const NAME: &'static str = "Fretcat";
    const VENDOR: &'static str = "GatoImorrivel";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "gsantos1510@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The SIMD version only supports stereo
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(NUM_INPUT_CHANNELS),
        main_output_channels: NonZeroU32::new(NUM_OUTPUT_CHANNELS),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        fretcat_editor::editor::create(
            fretcat_editor::editor::Data {
                noise_gate: self.noise_gate.clone()
            },
            ChainHandle::new(self.chain.as_ptr()),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel in buffer.iter_samples() {
            for sample in channel {
                for effect in &self.chain.get_mut().chain {
                    *sample = effect.process(*sample);
                }
            }
        }
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for Fretcat {
    const VST3_CLASS_ID: [u8; 16] = *b"FretcatGatoPlug.";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Filter,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_vst3!(Fretcat);
