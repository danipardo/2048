extern crate log;
extern crate simple_logger;

use rand::Rng;

use rand;
pub use self::Direction::{};
// use std::collections::HashSet;

pub struct Board {
    cells: [[u16; 4]; 4],
    score: u16,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,

}


pub fn actuate_board(board: &[[u16; 4]; 4]) -> [[u16; 4]; 4] {
    let mut result = [[0u16; 4]; 4];
    for x in 0..4 {
        result[x] = actuate_row(&board[x])
    }
    result
}

// Partint de la posicio n, i tirant cap a l'esquerre,
// busca el primer numero que no es zero
fn find_rev(input: &[u16; 4], n: usize) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut x = n - 1;
    while input.get(x).is_some() {
        let value = input.get(x).unwrap();
        if *value == 0 {
            if x == 0 {
                return None;
            }
            x = x - 1;
        } else {
            return Some(x);
        }
    }
    None
}

pub fn slide_row(input: &[u16; 4]) -> [u16; 4] {
    let mut result = [0u16; 4];
    let mut index = 3;
    for i in (0..4).rev() {
        if input[i] > 0 {
            result[index] = input[i];
            if (index > 0) {
                index = index - 1;
            }
        }
    }
    result
}

pub fn combine_row(input: &[u16; 4]) -> [u16; 4] {
    let mut result = input.clone();

    for i in (1..4).rev() {
        if result[i] == result[i - 1] {
            result[i] = result[i] * 2;
            result[i - 1] = 0
        }
    }
    result
}

pub fn actuate_row(input: &[u16; 4]) -> [u16; 4] {
    let mut result = slide_row(input);
    let mut result = combine_row(&result);
    let mut result = slide_row(&result);

    result
}

// Rotates the board 90 degrees clockwise
pub fn rotate(board: &[[u16; 4]; 4]) -> [[u16; 4]; 4] {
    let mut v2 = [[0u16; 4]; 4];
    for x in 0..4 {
        for y in 0..4 {
            v2[x][y] = board[y][x];
        }
    }

    invert(&v2)
}

pub fn invert(board: &[[u16; 4]; 4]) -> [[u16; 4]; 4] {
    let mut result = [[0u16; 4]; 4];
    for x in 0..4 {
        let mut row = board[x].clone();
        row.reverse();
        result[x] = row;
    }
    result
}

impl Board {
    pub fn new() -> Board {
        let board = Board { cells: [[0u16; 4]; 4], score: 0 };
        board
    }
    pub fn add_random_cell(&mut self) {
        let mut found = false;
        let mut x = 0;
        let mut y = 0;
        while !found {
            x = rand::thread_rng().gen_range(0, 4);
            y = rand::thread_rng().gen_range(0, 4);
            found = self.cells[x][y] == 0;
        }

        let mut rand : f32 = rand::thread_rng().gen();
        if rand > 0.5 {
            self.cells[x][y] = 2;
        } else {
            self.cells[x][y] = 4;
        }
    }
    pub fn set_cells(&mut self, input: [[u16; 4]; 4]) {
        self.cells = input
    }

    pub fn get_cell(&self, x: usize, y: usize) -> u16 {
        self.cells[x][y]
    }
    pub fn print(&self) {
        for i in 0..4 {
            println!("{:?}", &self.cells[i]);
        }
    }

    /// returns true if there is a movement in the board
    pub fn actuate(&mut self, direction: Direction) -> bool {
        let mut new_board = [[0u16; 4]; 4];

        match direction {
            Direction::Right => {
                new_board = actuate_board(&self.cells);
            }
            Direction::Down => {
                new_board = rotate(&self.cells);
                new_board = invert(&new_board);
                new_board = actuate_board(&new_board);
                new_board = invert(&new_board);
                new_board = rotate(&new_board);
                new_board = rotate(&new_board);
                new_board = rotate(&new_board);
            }
            Direction::Up => {
                new_board = rotate(&self.cells);
                new_board = actuate_board(&new_board);
                new_board = rotate(&new_board);
                new_board = rotate(&new_board);
                new_board = rotate(&new_board);
            }
            Direction::Left => {
                new_board = invert(&self.cells);
                new_board = actuate_board(&new_board);
                new_board = invert(&new_board);
            }
        }

        for x in 0..4 {
            for y in 0..4 {
                if new_board[x][y] != self.cells[x][y] {
                    self.cells = new_board;
                    return true;
                }
            }
        }

        false // no change applied to the board
    }
}