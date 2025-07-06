use std::f32;

use nih_plug::prelude::Enum;

#[derive(PartialEq, Clone)]
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

const IDX_SINE: usize = 0;
const IDX_SAWTOOTH: usize = 1;
const IDX_SQUARE: usize = 2;
const IDX_TRIANGLE: usize = 3;

impl Enum for Waveform {
    fn variants() -> &'static [&'static str] {
        &["Sine", "Square", "Triangle", "Sawtooth"]
    }

    fn ids() -> Option<&'static [&'static str]> {
        None
    }

    fn to_index(self) -> usize {
        match self {
            Self::Sine => IDX_SINE,
            Self::Sawtooth => IDX_SAWTOOTH,
            Self::Square => IDX_SQUARE,
            Self::Triangle => IDX_TRIANGLE,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            IDX_SINE => Self::Sine,
            IDX_SAWTOOTH => Self::Sawtooth,
            IDX_SQUARE => Self::Square,
            IDX_TRIANGLE => Self::Triangle,
            _ => panic!("Unexpected index"),
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
