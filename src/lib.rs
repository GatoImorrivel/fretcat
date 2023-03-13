pub mod editor;
pub mod effects;

use effects::Effect;
use fundsp::prelude::U1;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::Arc;

pub use nih_plug;

pub struct FretCat {
    params: Arc<FretCatParams>,
    loaded_effects: Vec<Effect>,
    prev_samples: Vec<f32>
}

#[derive(Params)]
struct FretCatParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "freq"]
    freq: FloatParam,
}

impl Default for FretCat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretCatParams::default()),
            loaded_effects: vec![],
            prev_samples: vec![],
        }
    }
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            freq: FloatParam::new(
                "Cutoff",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 44100.0,
                },
            )
            .with_unit(" Hz"),
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
        editor::create(self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.loaded_effects.push(effects::make_overdrive());
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
        let freq = self.params.freq.value();

        let filter: fundsp::prelude::ButterLowpass<f32, f32, U1> = fundsp::filter::ButterLowpass::new(44100.0, freq);

        for buffer in buffer.as_slice() {
            for sample in buffer.iter() {
               self.prev_samples.push(*sample);
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
