pub mod effects;
pub mod editor;

use nih_plug::{log::log, prelude::*, params::persist};
use nih_plug_iced::IcedState;
use std::sync::Arc;

pub use nih_plug;

pub struct FretCat {
    params: Arc<FretCatParams>,
}

#[derive(Params)]
struct FretCatParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "threshold"]
    pub threshold: FloatParam,

    #[id = "blend"]
    pub blend: FloatParam,
}

impl Default for FretCat {
    fn default() -> Self {
        Self {
            params: Arc::new(FretCatParams::default()),
        }
    }
}

impl Default for FretCatParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            threshold: FloatParam::new(
                "Threshold",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(0.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(0.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit("dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            blend: FloatParam::new(
                "Blend",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: util::db_to_gain(0.0),
                    max: util::db_to_gain(2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0)),
        }
    }
}

impl Plugin for FretCat {
    const NAME: &'static str = "FretCat";
    const VENDOR: &'static str = "Gato";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "gsantos1510@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Hello World");
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
        for channel_samples in buffer.iter_samples() {
            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();
            let threshold = self.params.threshold.smoothed.next();
            let blend = self.params.blend.smoothed.next();

            for sample in channel_samples {
                *sample = effects::overdrive(*sample, gain, threshold, blend);
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for FretCat {
    const VST3_CLASS_ID: [u8; 16] = *b"FretCatGatoVst..";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_vst3!(FretCat);