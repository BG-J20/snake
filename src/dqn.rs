use ndarray::{Array1, Array2};
use rand::Rng;
use serde::{ Deserialize, Serialize };
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct DQN {
    pub weights: Array2<f32>,
    pub input_dim: usize,
    pub output_dim: usize,
}
impl DQN {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = Array2::from_shape_fn((input_dim, output_dim), |_| rng.gen_range(-1.0..1.0));

        Self {
            weights,
            input_dim,
            output_dim,
        }
    }
    pub fn predict(&self, input: &Array1<f32>) -> Array1<f32> {
        input.dot(&self.weights)
    }
    pub fn choose_action(&self, state: &Array1<f32>) -> usize {
        let predictions = self.predict(state);
        predictions
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    pub fn save(&self, path: &str) {
        let json = serde_json::to_string(self).unwrap();
        fs::write(path, json).unwrap();
    }
    pub fn load(path: &str) -> Option<Self> {
        let json = fs::read_to_string(path).ok()?;
        let weights: Array2<f32> = serde_json::from_str(&json).ok()?;

        let (input_dim, output_dim) = weights.dim();

        Some(Self {
            weights,
            input_dim,
            output_dim,
        })
    }
    pub fn learn(&mut self, state: &[f32], action: usize, reward: f32, next_state: &[f32]) {
        let gamma = 0.9;
        let lr = 0.01;

        let state = Array1::from(state.to_vec());
        let next_state = Array1::from(next_state.to_vec());

        let q_values = self.weights.dot(&state);
        let mut q_values = q_values.to_vec();

        let next_q_values = self.weights.dot(&next_state);
        let max_next_q = next_q_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let target = reward + gamma * max_next_q;
        let error = target - q_values[action];
        q_values[action] += lr * error;

        for i in 0..self.input_dim {
            self.weights[[action, i]] += lr * error * state[i];
        }
    }
    pub fn act(&self, state: &Vec<f32>) -> usize {
        let epsilon = 0.1; // вероятность случайного действия
        let mut rng = rand::thread_rng();

        if rng.r#gen::<f32>() < epsilon {
            // случайное действие
            rng.gen_range(0..self.output_dim)
        } else {
            // жадное действие: выбираем argmax от Q(s, a)
            let s = Array2::from_shape_vec((1, self.input_dim), state.clone()).unwrap();
            let q_values = s.dot(&self.weights);
            q_values
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap()
        }
    }

}