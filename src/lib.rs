mod generator;
mod oscillator;
mod parameters;
mod synthesizer;
mod voice;
mod waveform;

use nih_plug::prelude::*;
use synthesizer::Synthesizer;

impl Vst3Plugin for Synthesizer {
    const VST3_CLASS_ID: [u8; 16] = *b"Harmonicity_VST3";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Mono,
    ];
}

nih_export_vst3!(Synthesizer);
