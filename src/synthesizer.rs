use nih_plug::prelude::*;
use rand_pcg::Pcg32;
use std::sync::Arc;

use super::parameters::SynthParameters;
use super::voice::Voice;

const MAX_VOICES: usize = 16;

pub struct Synthesizer {
    voices: [Option<Voice>; MAX_VOICES],
    params: Arc<SynthParameters>,
    next_voice_age: usize,
    phase_generator: Pcg32,
}

impl Synthesizer {
    fn start_voice(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        timing: u32,
        voice_id: Option<i32>,
        note: u8,
        channel: u8,
    ) -> &mut Voice {
        let voice_id = voice_id.unwrap_or_else(|| compute_fallback_voice_id(note, channel));
        let voice = Voice::new(voice_id, self.next_voice_age, note, channel);
        self.next_voice_age += 1;

        // find voice
        let position = self.voices.iter().position(|voice| voice.is_none());

        if let Some(position) = position {
            self.voices[position] = Some(voice);
            return self.voices[position].as_mut().unwrap();
        }

        // otherwise steal the (oldest) voice
        let old_voice = self
            .voices
            .iter_mut()
            .min_by_key(|v| v.as_ref().unwrap().age())
            .unwrap();

        Self::release_voice(context, timing, old_voice);
        *old_voice = Some(voice);
        return old_voice.as_mut().unwrap();
    }

    fn clean_released_voices(&mut self, context: &mut impl ProcessContext<Self>, timing: u32) {
        self.voices
            .iter_mut()
            .filter(|voice| voice.is_some())
            .filter(|voice| voice.as_ref().unwrap().is_released())
            .for_each(|voice| Self::release_voice(context, timing, voice))
    }

    fn release_voice(
        context: &mut impl ProcessContext<Self>,
        timing: u32,
        voice: &mut Option<Voice>,
    ) {
        let voice_ref = voice.as_ref().unwrap();
        context.send_event(NoteEvent::VoiceTerminated {
            timing,
            voice_id: Some(voice_ref.voice_id()),
            channel: voice_ref.channel(),
            note: voice_ref.note(),
        });
        *voice = None;
    }
}

impl Default for Synthesizer {
    fn default() -> Self {
        Self {
            voices: [0; MAX_VOICES].map(|_| None),
            params: Arc::new(Default::default()),
            next_voice_age: 0,
            phase_generator: create_phase_generator(),
        }
    }
}

impl Plugin for Synthesizer {
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

    // fn initialize(
    //     &mut self,
    //     _audio_io_layout: &AudioIOLayout,
    //     buffer_config: &BufferConfig,
    //     _context: &mut impl InitContext<Self>,
    // ) -> bool {
    //     let _sample_rate = buffer_config.sample_rate;
    //     true
    // }

    fn reset(&mut self) {
        self.phase_generator = create_phase_generator();
        self.voices.fill(None);
        self.next_voice_age = 0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}

fn create_phase_generator() -> Pcg32 {
    Pcg32::new(420, 1337)
}
