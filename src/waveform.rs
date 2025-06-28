use std::f32;

pub enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Waveform {
    // avoid dynamic dispatch for performance reason
    pub fn evaluate(&self, x: f32) -> f32 {
        match self {
            Self::Sawtooth => sawtooth(x),
            Self::Sine => sine(x),
            Self::Square => square(x),
            Self::Triangle => triangle(x),
        }
    }
}

fn sine(x: f32) -> f32 {
    (f32::consts::TAU * x).sin()
}

fn square(x: f32) -> f32 {
    if x < 0.5 { 1.0 } else { -1.0 }
}

fn triangle(x: f32) -> f32 {
    2.0 * (2.0 * x - 1.0).abs() - 1.0
}

fn sawtooth(x: f32) -> f32 {
    2.0 * x - 1.0
}
