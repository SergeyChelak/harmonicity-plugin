use nih_plug::{
    params::{FloatParam, Params},
    prelude::FloatRange,
};

#[derive(Params)]
pub struct HarmonicityParameters {
    #[id = "env_attack"]
    attack_time: FloatParam,
    #[id = "env_decay"]
    decay_time: FloatParam,
    #[id = "env_sustain"]
    sustain_level: FloatParam,
    #[id = "env_release"]
    release_time: FloatParam,
}

impl Default for HarmonicityParameters {
    fn default() -> Self {
        Self {
            attack_time: time_parameter("Attack", 200.0, 0.0, 2000.0),
            decay_time: time_parameter("Decay", 100.0, 0.0, 2000.0),
            sustain_level: sustain_parameter("Sustain", 0.85),
            release_time: time_parameter("Release", 100.0, 0.0, 2000.0),
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
