use nih_plug::{
    debug::nih_log,
    prelude::{Smoother, SmoothingStyle},
    util,
};

use crate::oscillator::Oscillator;

const TOL: f32 = 1e-10;
const ENVELOPE_ATTACK_LEVEL: f32 = 1.0;
pub const OSCILLATORS_COUNT: usize = 3;

#[derive(Clone)]
pub struct Voice {
    voice_id: i32,
    age: usize,
    note: MidiNote,
    oscillators: [Oscillator; OSCILLATORS_COUNT],
    state: VoiceState,
    amp_envelope: Smoother<f32>,
    envelope: Envelope,
    sample_rate: f32,
}

impl Voice {
    pub fn new(
        sample_rate: f32,
        voice_id: i32,
        age: usize,
        note: MidiNote,
        oscillators: [Oscillator; OSCILLATORS_COUNT],
        envelope: Envelope,
    ) -> Self {
        let mut voice = Self {
            sample_rate,
            voice_id,
            age,
            note,
            oscillators,
            amp_envelope: Smoother::new(SmoothingStyle::None),
            envelope,
            state: VoiceState::Attack,
        };
        voice.prepare();
        voice
    }

    fn prepare(&mut self) {
        self.amp_envelope.reset(self.envelope.start_level);
        self.update_amp_envelope_style(self.envelope.attack_time, ENVELOPE_ATTACK_LEVEL);
    }

    pub fn age(&self) -> usize {
        self.age
    }

    pub fn voice_id(&self) -> i32 {
        self.voice_id
    }

    pub fn note(&self) -> &MidiNote {
        &self.note
    }

    pub fn is_deaf(&self) -> bool {
        matches!(self.state, VoiceState::Deaf)
    }

    pub fn next_sample(&mut self) -> f32 {
        if self.is_deaf() {
            return 0.0;
        }
        let sample = self
            .oscillators
            .iter_mut()
            .map(|osc| osc.next_sample())
            .sum::<f32>();

        sample * self.amp_envelope.next() * self.note.velocity
    }

    pub fn choke(&mut self, voice_id: Option<i32>, channel: u8, note: u8) -> bool {
        if !self.is_relevant_voice(voice_id, channel, note) {
            return false;
        }
        nih_log!("[envelope] choke {channel} {note}");
        self.state = VoiceState::Deaf;
        true
    }

    pub fn release_note(&mut self, voice_id: Option<i32>, channel: u8, note: u8) {
        if !self.is_relevant_voice(voice_id, channel, note) {
            return;
        }
        nih_log!("[envelope] {channel} {note} releasing");
        self.state = VoiceState::Release;
        self.update_amp_envelope_style(self.envelope.release_time, 0.0);
    }

    fn is_relevant_voice(&self, voice_id: Option<i32>, channel: u8, note: u8) -> bool {
        voice_id == Some(self.voice_id) || self.note.is_matches(channel, note)
    }

    pub fn update_envelope(&mut self) {
        use VoiceState::*;
        let amp = self.amp_envelope.previous_value();
        match self.state {
            Attack if (amp - ENVELOPE_ATTACK_LEVEL).abs() < TOL => {
                nih_log!("[envelope] attack -> decay");
                self.state = Decay;
                self.update_amp_envelope_style(
                    self.envelope.decay_time,
                    self.envelope.sustain_level,
                );
            }
            Decay if (amp - self.envelope.sustain_level).abs() < TOL => {
                nih_log!("[envelope] decay -> sustain");
                self.state = Sustain;
            }
            Release if amp.abs() < TOL => {
                nih_log!("[envelope] release -> deaf");
                self.state = Deaf;
            }
            _ => {
                // no op
            }
        }
    }

    fn update_amp_envelope_style(&mut self, time: f32, target: f32) {
        self.amp_envelope.style = SmoothingStyle::Exponential(time);
        self.amp_envelope.set_target(self.sample_rate, target);
    }
}

#[derive(Clone)]
pub enum VoiceState {
    Attack,
    Decay,
    Sustain,
    Release,
    Deaf,
}

#[derive(Clone)]
pub struct Envelope {
    pub start_level: f32,
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_level: f32,
    pub release_time: f32,
}

#[derive(Debug, Clone)]
pub struct MidiNote {
    pub channel: u8,
    pub number: u8,
    pub velocity: f32,
}

impl MidiNote {
    pub fn source_id(&self) -> i32 {
        self.number as i32 | ((self.channel as i32) << 16)
    }

    fn is_matches(&self, channel: u8, note: u8) -> bool {
        self.channel == channel && self.number == note
    }

    pub fn frequency(&self) -> f32 {
        util::midi_note_to_freq(self.number)
    }
}
