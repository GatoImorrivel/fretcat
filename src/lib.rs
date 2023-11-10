mod params;

use fretcat_serialization::Preset;
pub use nih_plug;
use nih_plug::prelude::*;

use fretcat_effects::{Chain, NUM_CHANNELS};
use params::FretcatParams;

use std::{num::NonZeroU32, sync::{Arc, Mutex}};

pub struct Fretcat {
    params: Arc<FretcatParams>,
    chain: Arc<Chain>,
    preset: Arc<Mutex<Preset>>
}

impl Default for Fretcat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretcatParams::default()),
            chain: Arc::new(Chain::default()),
            preset: Arc::new(Mutex::new(Preset::default()))
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
        main_input_channels: NonZeroU32::new(NUM_CHANNELS as u32),
        main_output_channels: NonZeroU32::new(NUM_CHANNELS as u32),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;
    const HARD_REALTIME_ONLY: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        fretcat_editor::create(self.chain.clone(), self.preset.clone(), self.params.editor_state.clone())
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
        let transport = _context.transport();
        let chain = unsafe {
            &mut *Arc::as_ptr(&self.chain).cast_mut()
        };

        chain.process(buffer.as_slice(), transport);

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
