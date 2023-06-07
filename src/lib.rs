use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        return Snake {
            body: body,
            direction: Direction::Right,
        };
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
    status: Option<GameStatus>,
}
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let snake = Snake::new(snake_idx, 3);

        let reward_cell = World::gen_reward_cell(width, &snake.body);

        return World {
            width,
            snake,
            next_cell: None,
            reward_cell,
            status: None,
        };
    }

    fn gen_reward_cell(width: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
            // loop infinito
            reward_cell = random(width * width);
            // if !self.snake.body.contains(&SnakeCell(reward_cell)) {
            //     break;
            // }
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        return reward_cell;
    }

    pub fn width(&self) -> usize {
        return self.width; // Esses valores podem ser retornados diretamente pois são criados na stack
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head(&self) -> usize {
        return self.snake.body[0].0; // Pega o primeiro elemento de SnakeCell, ou seja, o unico que tem
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        return self.status;
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("Ganhou!"),
            Some(GameStatus::Lost) => String::from("Perdeu"),
            Some(GameStatus::Played) => String::from("Jogando"),
            None => String::from("Nenhum estado"),
        }
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return; // Caso ele queira mudar da esquerda para direita imediantamente, ou de cima para baixo
        }

        self.next_cell = Option::Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_len(&self) -> usize {
        return self.snake.body.len();
    }

    // * é um ponteiro puro, ou seja, ele não é gerenciado pelo Rust desta forma as regras de borrow checker não se aplicam
    pub fn snake_cells(&self) -> *const SnakeCell {
        return self.snake.body.as_ptr();
    }

    // Nao consigo retornar o objeto, pois ele vai ser destruido quando a funcao acabar
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //     return self.snake.body;
    // }

    pub fn update(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temp = self.snake.body.clone();

                // Estava dando um bug onde quando eu apertava varias vezes a mesma tecla a cobra ficava com os pedacos espaçados
                // match self.next_cell {
                //     Some(cell) => {
                //         self.snake.body[0] = cell;
                //         self.next_cell = None;
                //         return;
                //     }
                //     None => {
                //         self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                //     }
                // }

                let next_cell = self.gen_next_snake_cell(&self.snake.direction);
                self.snake.body[0] = next_cell;

                let len = self.snake.body.len();

                for i in 1..len {
                    // comeca em 1 pois ja atualizamos o 0 em self.snake.body[0] = next_cell;
                    // e tambem porque estamos pegando i-1, e para i = 0  teriamos -1
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_len()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                // Aqui aparentemente nao import porque quando rodar o for em cima vai pegar o proximo valor e vai sobrescrever
                if self.reward_cell == self.snake_head() {
                    if self.snake_len() < self.width * self.width {
                        self.reward_cell = World::gen_reward_cell(self.width, &self.snake.body);
                    } else {
                        // Tamanho da cobra é maior que o jogo, logo nao temos lugar para ter a reward
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }

                // let snake_idx: usize = self.snake_head();
                // // let row = snake_idx / self.width;
                // // let col = snake_idx % self.width;

                // let (row, col) = self.index_to_cell(snake_idx);

                // let (row, col) = match self.snake.direction {
                //     Direction::Right => (row, (col + 1) % self.width),
                //     Direction::Left => (row, (col - 1) % self.width),
                //     Direction::Up => ((row - 1) % self.width, col),
                //     Direction::Down => ((row + 1) % self.width, col),
                // };

                // let next_idx = self.cell_to_index(row, col);
                // self.set_snake_head(next_idx);

                // if self.snake.direction == Direction::Right {
                //     let next_col = (snake_idx + 1) % self.width;
                //     self.snake.body[0].0 = (row * self.width) + next_col;
                //     //? Mesma coisa que self.snake.body[0].0 = self.snake.body[0].0 + 1 % ...
                // } else if self.snake.direction == Direction::Left {
                //     self.snake.body[0].0 = (snake_idx - 1) % (self.width * self.width);
                // } else if self.snake.direction == Direction::Up {
                //     let next_row = (row - 1) % self.width;
                //     self.snake.body[0].0 = (next_row * self.width) + col;
                // } else {
                //     // Down
                //     let next_row = (row + 1) % self.width;
                //     self.snake.body[0].0 = (next_row * self.width) + col;
                // }
            }
            _ => {}
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx: usize = self.snake_head();
        let row = snake_idx / self.width;
        let size = self.width * self.width;

        return match direction {
            Direction::Right => {
                // Forma sem usar % que usa divisão e é mais 'Caro'
                let limite = (row + 1) * self.width;
                if snake_idx + 1 == limite {
                    return SnakeCell(limite - self.width);
                } else {
                    return SnakeCell(snake_idx + 1);
                }
            } //SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % size),
            Direction::Down => SnakeCell((snake_idx + self.width) % size),
        };
    }

    // fn set_snake_head(&mut self, idx: usize) {
    //     self.snake.body[0].0 = idx;
    // }

    // fn index_to_cell(&self, idx: usize) -> (usize, usize) {
    //     return (idx / self.width, idx % self.width);
    // }

    // fn cell_to_index(&self, row: usize, col: usize) -> usize {
    //     return (row * self.width) + col;
    // }
}
