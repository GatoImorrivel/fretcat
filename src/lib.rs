mod editor;
mod params;
mod effects;

use effects::{Effect, Overdrive};
use nih_plug::{prelude::*};
use params::FretCatParams;
use std::{sync::Arc, any::Any};

pub use nih_plug;

pub struct FretCat {
    params: Arc<FretCatParams>,
    chain: Vec<Box<dyn Effect + Send + Sync>>,
}

impl Default for FretCat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretCatParams::default()),
            chain: vec![Box::new(Overdrive::default())]
        }
    }
}

impl FretCat {
    pub fn update(&mut self) {
        let msg = self.params.ui_message.take();
        if msg.is_some() {
            let content = msg.unwrap();
            self.chain[content.get_id()].update(content.get_message());
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
        editor::create(self.params.editor_state.clone(), self.params.ui_message.clone())
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
        self.update();
        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                for effect in &self.chain {
                    *sample = effect.process(*sample);
                }
            }
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
