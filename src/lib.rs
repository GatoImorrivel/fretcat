mod editor;
mod chain;

use chain::EffectChain;
use crossbeam::atomic::AtomicCell;
use nih_plug::{prelude::*, params::persist};
use nih_plug_iced::IcedState;
use std::sync::Arc;

pub use nih_plug;

pub struct FretCat {
    params: Arc<FretCatParams>,
    chain: Arc<AtomicCell<EffectChain>>
}

#[derive(Params)]
struct FretCatParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
}

impl Default for FretCat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretCatParams::default()),
            chain: Arc::new(EffectChain::default().into())
        }
    }
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for FretCat {
    const NAME: &'static str = "FretCat";
    const VENDOR: &'static str = "Gato";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "gsantos1510@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.editor_state.clone(), self.chain.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for buffer in buffer.as_slice() {
            buffer.reverse();
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for FretCat {
    const VST3_CLASS_ID: [u8; 16] = *b"FretCatGatoVst..";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_vst3!(FretCat);
