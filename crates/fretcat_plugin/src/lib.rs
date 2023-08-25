mod params;

use fretcat_editor::EditorData;
pub use nih_plug;
use nih_plug::prelude::*;

use fretcat_effects::{AtomicRefCell, Chain, ChainHandle};
use fretcat_effects::rayon::prelude::*;
use params::FretcatParams;

use std::sync::atomic::Ordering;
use std::{num::NonZeroU32, sync::Arc};

const NUM_INPUT_CHANNELS: u32 = 2;
const NUM_OUTPUT_CHANNELS: u32 = 2;
pub struct Fretcat {
    params: Arc<FretcatParams>,
    chain: ChainHandle,
    editor_data: EditorData
}

impl Default for Fretcat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretcatParams::default()),
            chain: Arc::new(AtomicRefCell::new(Chain::default())),
            editor_data: EditorData::default()
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
    type BackgroundTask = ChainHandle;

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        #[allow(unused_parens)]
        fretcat_editor::create((self.chain.clone(), self.editor_data.clone()), self.params.editor_state.clone())
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

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        Box::new(|chain| {
             match chain.borrow().update_queue.pop() {
                Some(command) => {
                    unsafe {
                        chain.as_ptr().as_mut().unwrap().handle_command(command);
                    }
                },
                None => ()
            }
        })
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel in buffer.as_slice() {
            for effect in self.chain.borrow().effects.iter() {
                self.chain.borrow_mut().query(effect).unwrap().process(*channel);
            }
        }

        if !self.chain.borrow().update_queue.is_empty() {
            _context.execute_background(self.chain.clone());
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
