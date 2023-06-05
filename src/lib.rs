use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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
}
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        return World {
            width: width,
            snake: Snake::new(snake_idx, 3),
        };
    }

    pub fn width(&self) -> usize {
        return self.width; // Esses valores podem ser retornados diretamente pois são criados na stack
    }

    pub fn snake_head(&self) -> usize {
        return self.snake.body[0].0; // Pega o primeiro elemento de SnakeCell, ou seja, o unico que tem
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
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
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;
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

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx: usize = self.snake_head();
        let row = snake_idx / self.width;
        let size = self.width * self.width;

        return match self.snake.direction {
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
