mod oscillator;
mod parameters;
mod synthesizer;
mod voice;
mod waveform;

use nih_plug::prelude::*;
use synthesizer::Synthesizer;

// impl Plugin for Synthesizer {

//     fn process(
//         &mut self,
//         buffer: &mut Buffer,
//         _aux: &mut AuxiliaryBuffers,
//         context: &mut impl ProcessContext<Self>,
//     ) -> ProcessStatus {
//         for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
//             // process events
//             while let Some(event) = context.next_event() {
//                 if event.timing() > sample_id as u32 {
//                     break;
//                 }
//                 self.process_event(event);
//             }

//             // self.calculate_sine(self.midi_note_freq) * self.midi_note_gain.next()

//             let output = self.voice.next_sample();
//             for sample in channel_samples {
//                 *sample = output;
//             }
//         }
//         ProcessStatus::Normal
//     }
// }

// impl Harmonicity {
//     fn process_event(&mut self, event: NoteEvent<()>) {
//         use NoteEvent::*;
//         match event {
//             NoteOn {
//                 timing,
//                 voice_id,
//                 channel,
//                 note,
//                 velocity,
//             } => {
//                 //
//             }
//             NoteOff {
//                 timing,
//                 voice_id,
//                 channel,
//                 note,
//                 velocity,
//             } => {
//                 //
//             }
//             _ => {
//                 //
//             }
//         }
//     }
// }

impl Vst3Plugin for Synthesizer {
    const VST3_CLASS_ID: [u8; 16] = *b"Harmonicity_VST3";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Mono,
    ];
}

nih_export_vst3!(Synthesizer);
