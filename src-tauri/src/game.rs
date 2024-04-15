use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Game {
    pub state: [[u32; 4]; 4],
    pub score: u32,
    pub lock: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            state: [[0; 4]; 4],
            score: 0,
            lock: false,
        };
        game.random_fill();
        game.random_fill();
        game
    }

    fn random_fill(&mut self) {
        let mut free_cells: Vec<(usize, usize)> = Vec::new();
        for row in 0..4 {
            for col in 0..4 {
                if self.state[row][col] == 0 {
                    free_cells.push((row, col));
                }
            }
        }
        if !free_cells.is_empty() {
            let index = rand::thread_rng().gen_range(0..free_cells.len());
            self.state[free_cells[index].0][free_cells[index].1] = 2;
        }
    }

    pub fn event(&mut self, game_event: u8) {
        if !self.lock {
            match game_event {
                1 => self.move_to_up(),
                2 => self.move_to_right(),
                3 => self.move_to_down(),
                4 => self.move_to_left(),
                _ => {}
            }
            self.random_fill();
            if self.is_locked() {
                self.lock = true;
            }
        }
    }

    fn move_to_left(&mut self) {
        for row in 0..4 {
            for col in 1..=3 {
                for temp_col in (1..=col).rev() {
                    if self.state[row][temp_col - 1] == 0 && self.state[row][temp_col] != 0 {
                        self.state[row][temp_col - 1] = self.state[row][temp_col];
                        self.state[row][temp_col] = 0;
                    }
                }
            }
            for col in 1..=3 {
                if self.state[row][col - 1] == self.state[row][col] && self.state[row][col] != 0 {
                    self.state[row][col - 1] *= 2;
                    self.score += self.state[row][col - 1];
                    self.state[row][col] = 0;
                }
            }
            for col in 1..=3 {
                if self.state[row][col - 1] == 0 && self.state[row][col] != 0 {
                    self.state[row][col - 1] = self.state[row][col];
                    self.state[row][col] = 0;
                }
            }
        }
    }

    fn move_to_right(&mut self) {
        for row in 0..4 {
            for col in (0..=2).rev() {
                for temp_col in col..3 {
                    if self.state[row][temp_col + 1] == 0 && self.state[row][temp_col] != 0 {
                        self.state[row][temp_col + 1] = self.state[row][temp_col];
                        self.state[row][temp_col] = 0;
                    }
                }
            }
            for col in (0..=2).rev() {
                if self.state[row][col + 1] == self.state[row][col] && self.state[row][col] != 0 {
                    self.state[row][col + 1] *= 2;
                    self.score += self.state[row][col + 1];
                    self.state[row][col] = 0;
                }
            }
            for col in (0..=2).rev() {
                if self.state[row][col + 1] == 0 && self.state[row][col] != 0 {
                    self.state[row][col + 1] = self.state[row][col];
                    self.state[row][col] = 0;
                }
            }
        }
    }

    fn move_to_up(&mut self) {
        for col in 0..4 {
            for row in 1..=3 {
                for temp_row in (1..=row).rev() {
                    if self.state[temp_row - 1][col] == 0 && self.state[temp_row][col] != 0 {
                        self.state[temp_row - 1][col] = self.state[temp_row][col];
                        self.state[temp_row][col] = 0;
                    }
                }
            }
            for row in 1..=3 {
                if self.state[row - 1][col] == self.state[row][col] && self.state[row][col] != 0 {
                    self.state[row - 1][col] *= 2;
                    self.score += self.state[row - 1][col];
                    self.state[row][col] = 0;
                }
            }
            for row in 1..=3 {
                if self.state[row - 1][col] == 0 && self.state[row][col] != 0 {
                    self.state[row - 1][col] = self.state[row][col];
                    self.state[row][col] = 0;
                }
            }
        }
    }

    fn move_to_down(&mut self) {
        for col in 0..4 {
            for row in (0..=2).rev() {
                for temp_row in row..3 {
                    if self.state[temp_row + 1][col] == 0 && self.state[temp_row][col] != 0 {
                        self.state[temp_row + 1][col] = self.state[temp_row][col];
                        self.state[temp_row][col] = 0;
                    }
                }
            }
            for row in (0..=2).rev() {
                if self.state[row + 1][col] == self.state[row][col] && self.state[row][col] != 0 {
                    self.state[row + 1][col] *= 2;
                    self.score += self.state[row + 1][col];
                    self.state[row][col] = 0;
                }
            }
            for row in (0..=2).rev() {
                if self.state[row + 1][col] == 0 && self.state[row][col] != 0 {
                    self.state[row + 1][col] = self.state[row][col];
                    self.state[row][col] = 0;
                }
            }
        }
    }

    fn is_locked(&self) -> bool {
        for row in 0..=3 {
            for col in 0..=3 {
                if self.state[row][col] == 0
                    || row != 3 && self.state[row][col] == self.state[row + 1][col]
                    || col != 3 && self.state[row][col] == self.state[row][col + 1]
                {
                    return false;
                }
            }
        }
        true
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}
