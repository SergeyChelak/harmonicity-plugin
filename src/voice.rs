#[derive(Clone)]
pub struct Voice {
    voice_id: i32,
    age: usize,
    channel: u8,
    note_number: u8,
    velocity: u8,
    state: VoiceState,
}

impl Voice {
    pub fn new(voice_id: i32, age: usize, note_number: u8, channel: u8) -> Self {
        Self {
            voice_id,
            age,
            note_number,
            channel,
            velocity: 0,
            state: VoiceState::Attack,
        }
    }

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
}

#[derive(Clone)]
pub enum VoiceState {
    Attack,
    Releasing,
    Released,
}
