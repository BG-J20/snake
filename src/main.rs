use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    width: usize,
    height: usize,
    snake: VecDeque<(usize, usize)>,
    direction: Direction,
    food: (usize, usize),
    speed: Duration,
}

impl Game {
    fn new(width: usize, height: usize, speed_ms: u64) -> Self {
        let mut snake = VecDeque::new();
        let start_x = width / 2;
        let start_y = height / 2;
        snake.push_back((start_x, start_y));

        let food = Game::spawn_food(&snake, width, height);

        Self {
            width,
            height,
            snake,
            direction: Direction::Right,
            food,
            speed: Duration::from_millis(speed_ms),
        }
    }

    fn spawn_food(snake: &VecDeque<(usize, usize)>, width: usize, height: usize) -> (usize, usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if !snake.contains(&(x, y)) {
                return (x, y);
            }
        }
    }

    fn update_snake(&mut self) -> bool {
        let (head_x, head_y) = self.snake.front().unwrap();
        let (new_x, new_y) = match self.direction {
            Direction::Up => (*head_x, head_y.saturating_sub(1)),
            Direction::Down => (*head_x, *head_y + 1),
            Direction::Left => (head_x.saturating_sub(1), *head_y),
            Direction::Right => (head_x + 1, *head_y),
        };

        // Проверяем столкновение с границами
        if new_x >= self.width || new_y >= self.height || self.snake.contains(&(new_x, new_y)) {
            return false;
        }

        self.snake.push_front((new_x, new_y));

        if (new_x, new_y) == self.food {
            self.food = Game::spawn_food(&self.snake, self.width, self.height);
        } else {
            self.snake.pop_back();
        }
        true
    }

    fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");

        // Верхняя граница
        println!("{}", "#".repeat(self.width * 2 + 2));

        for y in 0..self.height {
            print!("#"); // Левая граница

            for x in 0..self.width {
                if self.snake.front() == Some(&(x, y)) {
                    print!("*");
                } else if self.snake.contains(&(x, y)) {
                    print!("&");
                } else if self.food == (x, y) {
                    print!("@");
                } else {
                    print!("  "); // 2 пробела для выравнивания
                }
            }

            print!("#"); // Правая граница
            println!();
        }

        // Нижняя граница
        println!("{}", "#".repeat(self.width * 2 + 1));
    }

    fn set_direction(&mut self, dir: Direction) {
        use Direction::*;

        if (self.direction == Up && dir != Down)
            || (self.direction == Down && dir != Up)
            || (self.direction == Left && dir != Right)
            || (self.direction == Right && dir != Left)
        {
            self.direction = dir;
        }
    }
}

fn get_user_input(prompt: &str) -> usize {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or(10)
}

fn main() {
    let width = get_user_input("What is width : ");
    let height = get_user_input("What is height : ");
    let speed = get_user_input("What is speed : ");

    let mut game = Game::new(width, height, speed as u64);

    let mut stdin = io::stdin();
    let mut input = [0; 1];

    loop {
        let start = Instant::now();

        if stdin.read(&mut input).is_ok() {
            match input[0] as char {
                'w' => game.set_direction(Direction::Up),
                's' => game.set_direction(Direction::Down),
                'a' => game.set_direction(Direction::Left),
                'd' => game.set_direction(Direction::Right),
                _ => {}
            }
        }

        if !game.update_snake() {
            println!("💀 Game Over! Длина змеи: {}", game.snake.len());
            break;
        }

        game.draw();

        let elapsed = start.elapsed();
        if game.speed > elapsed {
            sleep(game.speed - elapsed);
        }
    }
}
