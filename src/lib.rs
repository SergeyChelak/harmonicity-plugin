mod oscillator;
mod parameters;
mod voice;
mod waveform;

use nih_plug::prelude::*;
use parameters::*;
use std::sync::Arc;

pub struct Harmonicity {
    params: Arc<HarmonicityParameters>,
}

impl Default for Harmonicity {
    fn default() -> Self {
        Self {
            params: Arc::new(Default::default()),
        }
    }
}

impl Plugin for Harmonicity {
    const NAME: &'static str = "Harmonicity Synthesizer";

    const VENDOR: &'static str = "Sergey Chelak";

    const URL: &'static str = "https://github.com/SergeyChelak/harmonicity-plugin";

    const EMAIL: &'static str = "N/A";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(1),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();

    type BackgroundTask = ();

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    fn params(&self) -> std::sync::Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        todo!()
    }
}

impl Vst3Plugin for Harmonicity {
    const VST3_CLASS_ID: [u8; 16] = *b"Harmonicity_VST3";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Mono,
    ];
}

nih_export_vst3!(Harmonicity);
