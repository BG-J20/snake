use rand::Rng;

pub struct RandomAgent;

impl RandomAgent {
    pub fn new() -> Self {
        Self
    }

    pub fn act(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..4)
    }
}