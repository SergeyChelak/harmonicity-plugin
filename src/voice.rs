use nih_plug::{
    debug::nih_log,
    prelude::{Smoother, SmoothingStyle},
    util,
};

use crate::{parameters::SynthParameters, waveform::Waveform};

const TOL: f32 = 1e-10;
const ENVELOPE_ATTACK_LEVEL: f32 = 1.0;

pub struct VoiceBuilder {
    sample_rate: f32,
    voice_id: Option<i32>,
    note: u8,
    channel: u8,
    velocity: f32,
    age: usize,
    waveform: Waveform,
    phase: f32,
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
}

impl VoiceBuilder {
    pub fn new(sample_rate: f32, params: &SynthParameters) -> Self {
        Self {
            sample_rate,
            voice_id: None,
            channel: 0,
            note: 0,
            velocity: 1.0,
            age: 0,
            waveform: params.oscillator_1.waveform.value(),
            phase: 0.0,
            attack_time: params.envelope.attack_time.value(),
            decay_time: params.envelope.decay_time.value(),
            sustain_level: params.envelope.sustain_level.value(),
            release_time: params.envelope.release_time.value(),
        }
    }

    pub fn voice_id(mut self, voice_id: Option<i32>) -> Self {
        self.voice_id = voice_id;
        self
    }

    pub fn channel_note(mut self, channel: u8, note: u8) -> Self {
        self.channel = channel;
        self.note = note;
        self
    }

    pub fn velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn age(mut self, age: usize) -> Self {
        self.age = age;
        self
    }

    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }

    pub fn build(self) -> Voice {
        let voice_id = self
            .voice_id
            .unwrap_or_else(|| compute_fallback_voice_id(self.note, self.channel));
        let phase_delta = util::midi_note_to_freq(self.note) / self.sample_rate;

        let amp_envelope = Smoother::new(SmoothingStyle::Exponential(self.attack_time));
        amp_envelope.reset(0.0);
        amp_envelope.set_target(self.sample_rate, ENVELOPE_ATTACK_LEVEL);

        Voice {
            voice_id,
            age: self.age,
            channel: self.channel,
            note_number: self.note,
            velocity: self.velocity.sqrt(),
            state: VoiceState::Attack,
            waveform: self.waveform,
            phase: self.phase,
            phase_delta,
            amp_envelope,
            decay_time: self.decay_time,
            sustain_level: self.sustain_level,
            release_time: self.release_time,
            sample_rate: self.sample_rate,
            // TODO: fix hardcoded value
            gain: 0.3,
        }
    }
}

#[derive(Clone)]
pub struct Voice {
    voice_id: i32,
    age: usize,
    channel: u8,
    note_number: u8,
    velocity: f32,
    state: VoiceState,
    waveform: Waveform,
    phase: f32,
    phase_delta: f32,
    amp_envelope: Smoother<f32>,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    sample_rate: f32,
    gain: f32,
}

impl Voice {
    pub fn age(&self) -> usize {
        self.age
    }

    pub fn voice_id(&self) -> i32 {
        self.voice_id
    }

    pub fn note(&self) -> u8 {
        self.note_number
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn is_deaf(&self) -> bool {
        matches!(self.state, VoiceState::Deaf)
    }

    pub fn next_sample(&mut self) -> f32 {
        if self.is_deaf() {
            return 0.0;
        }
        let sample = self.waveform.evaluate(self.phase);
        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample * self.velocity * self.amp_envelope.next() * self.gain
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
        self.update_amp_envelope_style(self.release_time, 0.0);
    }

    fn is_relevant_voice(&self, voice_id: Option<i32>, channel: u8, note: u8) -> bool {
        voice_id == Some(self.voice_id) || self.channel == channel && self.note_number == note
    }

    pub fn update_envelope(&mut self) {
        use VoiceState::*;
        let amp = self.amp_envelope.previous_value();
        match self.state {
            Attack if (amp - ENVELOPE_ATTACK_LEVEL).abs() < TOL => {
                nih_log!("[envelope] attack -> decay");
                self.state = Decay;
                self.update_amp_envelope_style(self.decay_time, self.sustain_level);
            }
            Decay if (amp - self.sustain_level).abs() < TOL => {
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

const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}
