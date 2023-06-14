mod editor;
mod params;
mod effect;
mod chain;

use chain::{Chain, ChainPtr};
pub use nih_plug::{
    nih_export_vst3,
    prelude::{
        AsyncExecutor, AudioIOLayout, AuxiliaryBuffers, Buffer, BufferConfig, InitContext, Params,
        Plugin, PluginState, ProcessContext, ProcessStatus, Vst3Plugin, Vst3SubCategory, Editor,
    },
};
use params::FretcatParams;
use std::{num::NonZeroU32, sync::Arc, cell::Cell};

const NUM_INPUT_CHANNELS: u32 = 1;
const NUM_OUTPUT_CHANNELS: u32 = 2;

pub struct Fretcat {
    params: Arc<FretcatParams>,
    chain: Cell<Chain>
}

impl Default for Fretcat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretcatParams::default()),
            chain: Cell::new(Chain::default())
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
        editor::create(
            editor::Data {
                params: self.params.clone(),
                chain_ptr: ChainPtr::new(self.chain.as_ptr())
            },
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
