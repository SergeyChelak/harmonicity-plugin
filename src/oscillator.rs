use crate::waveform::Waveform;

#[derive(Clone)]
pub struct Oscillator {
    waveform: Waveform,
    phase: f32,
    phase_delta: f32,
    gain: f32,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sine,
            phase: Default::default(),
            phase_delta: Default::default(),
            gain: Default::default(),
        }
    }
}

impl Oscillator {
    pub fn new(waveform: Waveform, gain: f32, phase: f32, phase_delta: f32) -> Self {
        Oscillator {
            waveform,
            phase,
            phase_delta,
            gain,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.waveform.evaluate(self.phase);
        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample * self.gain
    }
}
