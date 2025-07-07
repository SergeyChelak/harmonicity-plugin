use nih_plug::{
    params::{EnumParam, FloatParam, Params},
    prelude::FloatRange,
};

use crate::waveform::Waveform;

#[derive(Params, Default)]
pub struct SynthParameters {
    #[nested(group = "Envelope")]
    pub envelope: EnvelopeParams,
    #[nested(group = "Oscillator 1")]
    pub oscillator_1: OscillatorParams,
}

#[derive(Params)]
pub struct EnvelopeParams {
    #[id = "env_attack"]
    pub attack_time: FloatParam,
    #[id = "env_decay"]
    pub decay_time: FloatParam,
    #[id = "env_sustain"]
    pub sustain_level: FloatParam,
    #[id = "env_release"]
    pub release_time: FloatParam,
}

impl Default for EnvelopeParams {
    fn default() -> Self {
        Self {
            attack_time: time_parameter("Attack", 25.0, 0.0, 2000.0),
            decay_time: time_parameter("Decay", 15.0, 0.0, 2000.0),
            sustain_level: sustain_parameter("Sustain", 0.85),
            release_time: time_parameter("Release", 10.0, 0.0, 2000.0),
        }
    }
}

#[derive(Params)]
pub struct OscillatorParams {
    #[id = "oscillator_wave_form"]
    pub waveform: EnumParam<Waveform>,
}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self {
            waveform: EnumParam::new("Waveform", Waveform::Sine),
        }
    }
}

fn time_parameter(name: &str, default: f32, min: f32, max: f32) -> FloatParam {
    let range = FloatRange::Skewed {
        min,
        max,
        factor: FloatRange::skew_factor(-1.0),
    };
    FloatParam::new(name, default, range)
        .with_step_size(0.1)
        .with_unit(" ms")
}

fn sustain_parameter(name: &str, default: f32) -> FloatParam {
    let range = FloatRange::Linear { min: 0.0, max: 1.0 };
    FloatParam::new(name, default, range)
        .with_step_size(0.05)
        .with_unit(" %")
}
