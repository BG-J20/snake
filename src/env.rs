use crate::{Direction, Game, WIDTH, HEIGHT};

pub struct Env {
    pub game: Game,
}

impl Env {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
        }
    }

    //возврат текущего состояния игры
    pub fn get_state(&self) -> Vec<f32> {
        let head = self.game.snake.front().unwrap();
        let food = self.game.food;

        vec![
            head.0 as f32 / WIDTH as f32,
            head.1 as f32 / HEIGHT as f32,
            food.0 as f32 / WIDTH as f32,
            food.1 as f32 / HEIGHT as f32,
            self.game.direction_to_vec().0,
            self.game.direction_to_vec().1,
        ]
    }

    // Выполняет шаг действия и возвращает (новое состояние, награда, завершена ли игра)
    pub fn step(&mut self, action: usize) -> (Vec<f32>, f32, bool) {
        let dir = match action {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => self.game.direction,
        };
        self.game.set_direction(dir);

        let  sucess = self.game.update();
        let reward = if !sucess {
            -1.0
        } else if self.game.snake.front().unwrap() == &self.game.food {
            1.0
        } else {
            0.0
        };

        (self.get_state(), reward, !sucess)
    }

    pub fn reset(&mut self) {
        self.game = Game::new();
    }
}