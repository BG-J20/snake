use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::{collections::VecDeque, time::{Duration, Instant}};

const CELL_SIZE: usize = 20;
const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const WINDOW_WIDTH: usize = WIDTH * CELL_SIZE;
const WINDOW_HEIGHT: usize = HEIGHT * CELL_SIZE;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: VecDeque<(usize, usize)>,
    direction: Direction,
    food: (usize, usize),
}

impl Game {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((WIDTH / 2, HEIGHT / 2));
        let food = Game::spawn_food(&snake);
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

    fn update(&mut self) -> bool {
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
            self.food = Game::spawn_food(&self.snake);
        } else {
            self.snake.pop_back();
        }

        true
    }

    fn set_direction(&mut self, new_dir: Direction) {
        use Direction::*;
        if (self.direction == Up && new_dir != Down)
            || (self.direction == Down && new_dir != Up)
            || (self.direction == Left && new_dir != Right)
            || (self.direction == Right && new_dir != Left)
        {
            self.direction = new_dir;
        }
    }
}

fn draw_game(buffer: &mut Vec<u32>, game: &Game) {
    buffer.fill(0); // Чистим экран (черный)

    // Змейка — зелёная
    for &(x, y) in &game.snake {
        draw_cell(buffer, x, y, 0x00FF00);
    }

    // Еда — красная
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
    let mut window = Window::new("🟢 Snake", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions::default()).unwrap();
    let mut buffer = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut game = Game::new();
    let mut last_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= Duration::from_millis(100) {
            if !game.update() {
                println!("Game Over! 🪦 Length: {}", game.snake.len());
                break;
            }
            draw_game(&mut buffer, &game);
            window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
            last_update = Instant::now();
        }

        // Управление
        if window.is_key_down(Key::W) { game.set_direction(Direction::Up); }
        if window.is_key_down(Key::S) { game.set_direction(Direction::Down); }
        if window.is_key_down(Key::A) { game.set_direction(Direction::Left); }
        if window.is_key_down(Key::D) { game.set_direction(Direction::Right); }
    }
}
