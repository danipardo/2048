extern crate log;
extern crate simple_logger;

use rand::Rng;

use rand;
pub use self::Direction::{};

pub struct Board {
    cells: [[u8; 4]; 4],
    score: u16,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,

}


pub fn actuate_board(board: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut result = [[0u8; 4]; 4];
    for x in 0..4 {
        result[x] = actuate_row(&board[x])
    }
    result
}

pub fn actuate_row(input: &[u8; 4]) -> [u8; 4] {
    let mut result: [u8; 4] = [0u8; 4];
    let mut actuated = false;
    for i in (1..4).rev() {
        let value = input[i];

        if value == 0 && input[i - 1] > 0 {
            result[i] = input[i - 1];
            actuated = true;
        }
        if value > 0 && input[i - 1] == value {
            result[i] = value * 2;
            result[i - 1] = 0;
            actuated = true;
        }
        if !actuated && value > 0 && input[i - 1] != value {
            result[i] = input[i];
        }
    }

    if actuated {
        result = actuate_row(&result);
    }

    result
}

pub fn rotate(board: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut v2 = [[0u8; 4]; 4];
    for x in 0..4 {
        for y in 0..4 {
            v2[x][y] = board[y][x];
        }
    }

    invert(&v2)
}

pub fn invert(board: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut result = [[0u8; 4]; 4];
    for x in 0..4 {
        let mut row = board[x].clone();
        row.reverse();
        result[x] = row;
    }
    result
}

impl Board {
    pub fn new() -> Board {
        let board = Board { cells: [[0u8; 4]; 4], score: 0 };
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

        self.cells[x][y] = 2;
    }
    pub fn set_cells(&mut self, input: [[u8; 4]; 4]) {
        self.cells = input
    }

    pub fn get_cell(&self, x: usize, y: usize) -> u8 {

        self.cells[x][y]

    }
    pub fn print(&self) {
        for i in 0..4 {
            println!("{:?}", &self.cells[i]);
        }
    }

    /// returns true if there is a movement in the board
    pub fn actuate(&mut self, direction: Direction) -> bool {
        let mut new_board = [[0u8; 4]; 4];

        match direction {
            Direction::Right => {
                new_board = actuate_board(&self.cells);
            }
            Direction::Down => {
                new_board = rotate(&self.cells);
                new_board = invert(&new_board);
                new_board = actuate_board(&new_board);
                new_board = invert(&new_board);
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