use nih_plug::util;

use crate::waveform::Waveform;

pub struct VoiceBuilder {
    sample_rate: f32,
    voice_id: Option<i32>,
    note: u8,
    channel: u8,
    velocity: f32,
    age: usize,
    waveform: Waveform,
    phase: f32,
}

impl VoiceBuilder {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            voice_id: None,
            channel: 0,
            note: 0,
            velocity: 1.0,
            age: 0,
            waveform: Waveform::Sine,
            phase: 0.0,
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

    pub fn waveform(mut self, waveform: Waveform) -> Self {
        self.waveform = waveform;
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
        Voice {
            voice_id,
            age: self.age,
            channel: self.channel,
            note_number: self.note,
            velocity: self.velocity,
            state: VoiceState::Attack,
            waveform: self.waveform,
            phase: self.phase,
            phase_delta,
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

    pub fn is_released(&self) -> bool {
        matches!(self.state, VoiceState::Released)
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.waveform.evaluate(self.phase);

        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

#[derive(Clone)]
pub enum VoiceState {
    Attack,
    Releasing,
    Released,
}

const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}
