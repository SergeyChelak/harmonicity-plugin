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
    ) {
        nih_log!("[synth] start voice for {channel}:{note}");

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
            nih_log!("[synth] voice at {position}");
            self.voices[position] = Some(voice);
            return;
        }

        // otherwise steal the (oldest) voice
        let Some(old_voice) = self
            .voices
            .iter_mut()
            .min_by_key(|v| v.as_ref().unwrap().age())
        else {
            nih_log!("[synth] failed to find voice for stealing");
            return;
        };

        nih_log!(
            "[synth] stealing voice with {}:{}",
            old_voice.as_ref().unwrap().channel(),
            old_voice.as_ref().unwrap().note()
        );
        Self::terminate_voice(context, timing, old_voice);
        *old_voice = Some(voice);
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
            .parameters(&self.params)
            .build()
    }

    fn release_voice(&mut self, voice_id: Option<i32>, note: u8, channel: u8) {
        self.voices
            .iter_mut()
            .filter_map(|v| v.as_mut())
            .for_each(|v| v.release_note(voice_id, channel, note));
    }

    fn next_age(&mut self) -> usize {
        let age = self.next_voice_age;
        self.next_voice_age += 1;
        age
    }

    fn update_envelopes(&mut self) {
        self.voices
            .iter_mut()
            .filter_map(|voice| voice.as_mut())
            .for_each(|voice| voice.update_envelope());
    }

    fn clean_released_voices(&mut self, context: &mut impl ProcessContext<Self>, timing: u32) {
        self.voices
            .iter_mut()
            .filter(|voice| voice.is_some())
            .filter(|voice| voice.as_ref().unwrap().is_released())
            .for_each(|voice| Self::terminate_voice(context, timing, voice))
    }

    fn terminate_voice(
        context: &mut impl ProcessContext<Self>,
        timing: u32,
        voice: &mut Option<Voice>,
    ) {
        let voice_ref = voice.as_ref().unwrap();
        nih_log!(
            "[synth] terminating voice with {}:{}",
            voice_ref.channel(),
            voice_ref.note()
        );

        context.send_event(NoteEvent::VoiceTerminated {
            timing,
            voice_id: Some(voice_ref.voice_id()),
            channel: voice_ref.channel(),
            note: voice_ref.note(),
        });
        *voice = None;
    }

    fn process_event(&mut self, context: &mut impl ProcessContext<Self>, event: NoteEvent<()>) {
        match event {
            NoteEvent::NoteOn {
                timing,
                voice_id,
                channel,
                note,
                velocity,
            } => {
                self.start_voice(context, timing, voice_id, note, channel, velocity);
            }
            NoteEvent::NoteOff {
                voice_id,
                channel,
                note,
                ..
            } => self.release_voice(voice_id, note, channel),
            NoteEvent::Choke {
                timing,
                voice_id,
                channel,
                note,
            } => {
                self.voices.iter_mut().for_each(|v| {
                    let Some(voice) = v else {
                        return;
                    };
                    if voice.choke(voice_id, channel, note) {
                        Self::terminate_voice(context, timing, v);
                    }
                });
            }
            _ => {
                // no op
                nih_log!("[synth] unhandled event");
            }
        }
    }

    fn render_sound(&mut self, output: &mut [&mut [f32]], block_start: usize, block_end: usize) {
        // We'll start with silence, and then add the output from the active voices
        output[0][block_start..block_end].fill(0.0);
        // output[1][block_start..block_end].fill(0.0);

        for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
            for sample_idx in block_start..block_end {
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
        nih_log!("[synth] will reset");

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

        let mut next_event = context.next_event();
        while block_start < samples_count {
            // process events
            'events: loop {
                match next_event {
                    Some(event) if event.timing() as usize <= block_start => {
                        nih_log!("[synth] got event {:?}", event);
                        self.process_event(context, event);
                        next_event = context.next_event();
                    }
                    Some(event) if (event.timing() as usize) < block_end => {
                        block_end = event.timing() as usize;
                        break 'events;
                    }
                    _ => break 'events,
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
