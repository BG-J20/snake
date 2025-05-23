mod agent;
mod env;
mod dqn;

use agent::RandomAgent;
use env::Env;
use rand::Rng;

use minifb::{Key, Window, WindowOptions};
use std::{time::{Duration, Instant}};

const CELL_SIZE: usize = 20;
pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 20;
const WINDOW_WIDTH: usize = WIDTH * CELL_SIZE;
const WINDOW_HEIGHT: usize = HEIGHT * CELL_SIZE;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::VecDeque;

pub struct Game {
    pub snake: VecDeque<(usize, usize)>,
    pub direction: Direction,
    pub food: (usize, usize),
}

impl Game {
    pub fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((WIDTH / 2, HEIGHT / 2));
        let food = Self::spawn_food(&snake);
        Self {
            snake,
            direction: Direction::Right,
            food,
        }
    }

    fn spawn_food(snake: &VecDeque<(usize, usize)>) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            if !snake.contains(&(x, y)) {
                return (x, y);
            }
        }
    }

    pub fn update(&mut self) -> bool {
        let (head_x, head_y) = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (*head_x, head_y.saturating_sub(1)),
            Direction::Down => (*head_x, head_y + 1),
            Direction::Left => (head_x.saturating_sub(1), *head_y),
            Direction::Right => (head_x + 1, *head_y),
        };

        if new_head.0 >= WIDTH || new_head.1 >= HEIGHT || self.snake.contains(&new_head) {
            return false;
        }

        self.snake.push_front(new_head);
        if new_head == self.food {
            self.food = Self::spawn_food(&self.snake);
        } else {
            self.snake.pop_back();
        }

        true
    }

    pub fn set_direction(&mut self, new_dir: Direction) {
        use Direction::*;
        if (self.direction == Up && new_dir != Down)
            || (self.direction == Down && new_dir != Up)
            || (self.direction == Left && new_dir != Right)
            || (self.direction == Right && new_dir != Left)
        {
            self.direction = new_dir;
        }
    }

    pub fn direction_to_vec(&self) -> (f32, f32) {
        match self.direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        }
    }
}

fn draw_game(buffer: &mut Vec<u32>, game: &Game) {
    buffer.fill(0);
    for &(x, y) in &game.snake {
        draw_cell(buffer, x, y, 0x00FF00);
    }
    draw_cell(buffer, game.food.0, game.food.1, 0xFF0000);
}

fn draw_cell(buffer: &mut Vec<u32>, x: usize, y: usize, color: u32) {
    for dy in 0..CELL_SIZE {
        for dx in 0..CELL_SIZE {
            let px = x * CELL_SIZE + dx;
            let py = y * CELL_SIZE + dy;
            if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                buffer[py * WINDOW_WIDTH + px] = color;
            }
        }
    }
}

fn main() {
    let mut window = Window::new("🤖 Snake AI", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()).unwrap();
    let mut buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut env = Env::new();
    let mut agent = crate::dqn::DQN::load("agent.json").unwrap_or_else(|| crate::dqn::DQN::new(6, 4));
    let mut rng = rand::thread_rng();

    let mut state = env.get_state();
    let mut last_update = Instant::now();
    let mut steps = 0;
    let mut episode_reward = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= Duration::from_millis(100) {
            let action = agent.act(&state);
            let (next_state, reward, done) = env.step(action);
            agent.learn(&state, action, reward, &next_state);
            state = if done {
                println!("🏁 Эпизод завершён. Награда: {}", episode_reward);
                episode_reward = 0.0;
                env.reset();
                env.get_state()
            } else {
                episode_reward += reward;
                next_state
            };

            // Отрисовка
            draw_game(&mut buffer, &env.game);
            window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
            last_update = Instant::now();

            steps += 1;
            if steps % 1000 == 0 {
                agent.save("agent.json");
                println!("💾 Сохранил веса в файл");
            }
        }
    }

    agent.save("agent.json");
    println!("💾 Финальное сохранение весов");
}

