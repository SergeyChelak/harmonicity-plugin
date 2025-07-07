use rand::Rng;
use rand_pcg::Pcg32;

pub struct Generator {
    pcg: Pcg32,
}

impl Generator {
    pub fn new() -> Self {
        Self { pcg: create_pcg() }
    }

    pub fn reset(&mut self) {
        self.pcg = create_pcg();
    }

    pub fn random(&mut self) -> f32 {
        self.pcg.random()
    }
}

fn create_pcg() -> Pcg32 {
    Pcg32::new(420, 1337)
}
