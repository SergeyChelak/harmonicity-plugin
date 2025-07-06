use nih_plug::prelude::*;
use rand::Rng;
use rand_pcg::Pcg32;
use std::sync::Arc;

use crate::voice::VoiceBuilder;

use super::parameters::SynthParameters;
use super::voice::Voice;

const MAX_VOICES: usize = 16;

const MAX_BLOCK_SIZE: usize = 64;

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
        velocity: f32,
    ) -> &mut Voice {
        let voice = self.make_voice(
            context.transport().sample_rate,
            voice_id,
            channel,
            note,
            velocity,
        );

        // find empty slot
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

    fn make_voice(
        &mut self,
        sample_rate: f32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        velocity: f32,
    ) -> Voice {
        VoiceBuilder::new(sample_rate)
            .channel_note(channel, note)
            .age(self.next_age())
            .voice_id(voice_id)
            .phase(self.phase_generator.random())
            .velocity(velocity)
            .build()
    }

    fn next_age(&mut self) -> usize {
        let age = self.next_voice_age;
        self.next_voice_age += 1;
        age
    }

    fn update_envelopes(&mut self) {
        // TODO: implement
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

    fn process_event(&mut self, event: NoteEvent<()>) {
        //
    }

    fn render_sound(&mut self, output: &mut [&mut [f32]], block_start: usize, block_end: usize) {
        // We'll start with silence, and then add the output from the active voices
        output[0][block_start..block_end].fill(0.0);
        // output[1][block_start..block_end].fill(0.0);

        // let block_len = block_end - block_start;
        for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
            // TODO: amp envelope
            for (idx, sample_idx) in (block_start..block_end).enumerate() {
                let sample = voice.next_sample();
                output[0][sample_idx] += sample;
            }
        }
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
        let samples_count = buffer.samples();
        let output = buffer.as_slice();
        let mut block_start = 0usize;
        let mut block_end = MAX_BLOCK_SIZE.min(samples_count);

        while block_start < samples_count {
            // process events
            while let Some(event) = context.next_event() {
                let timing = event.timing() as usize;
                if timing <= block_start {
                    self.process_event(event);
                    continue;
                }
                if timing < block_end {
                    block_end = timing;
                    break;
                }
            }
            self.render_sound(output, block_start, block_end);
            self.update_envelopes();
            self.clean_released_voices(context, block_end as u32);
            //
            block_start = block_end;
            block_end = (block_start + MAX_BLOCK_SIZE).min(samples_count);
        }

        ProcessStatus::Normal
    }
}

fn create_phase_generator() -> Pcg32 {
    Pcg32::new(420, 1337)
}
