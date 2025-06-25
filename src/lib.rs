use nih_plug::prelude::*;

pub struct HarmonicityPlugin {
    // no fields yet
}

impl Default for HarmonicityPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for HarmonicityPlugin {
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
        todo!()
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

impl Vst3Plugin for HarmonicityPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"Harmonicity_VST3";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Mono,
    ];
}

nih_export_vst3!(HarmonicityPlugin);
