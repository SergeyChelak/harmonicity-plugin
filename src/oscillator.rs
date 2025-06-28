use crate::waveform::Waveform;

pub struct Oscillator {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
    phase_delta: f32,
    waveform: Waveform,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            sample_rate: 0.0,
            frequency: 0.0,
            phase: 0.0,
            phase_delta: 0.0,
            waveform: Waveform::Sine,
        }
    }
}

impl Oscillator {
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase_delta = self.frequency / sample_rate
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase = 0.0;
        self.phase_delta = frequency / self.sample_rate
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.waveform.evaluate(self.phase);

        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase_delta -= 1.0;
        }

        sample
    }
}
