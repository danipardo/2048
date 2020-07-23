
extern crate sdl2;
mod resource_manager;
mod game;
mod board;

use board::Board;

use sdl2::pixels::Color;

use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;

pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("2048", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let mut texture_creator = canvas.texture_creator();
    let mut font_context = sdl2::ttf::init().unwrap();

    let mut game = game::Game::new(Box::new(event_pump),
                                   &mut texture_creator, &mut font_context);


    let c = &mut canvas;

    let mut board = Board::new();

    board.add_random_cell();
    while !game.finished() {
        game.draw(c, &board);
        game.update(timer.ticks());
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_actuate1() {
        let mut board = board::Board::new();
        assert_eq!(board::actuate_row(&[0, 4, 0, 0]), [0, 0, 0, 4]);
    }

    #[test]
    fn test_actuate2() {
        let mut board = board::Board::new();
        assert_eq!(board::actuate_row(&[0, 0, 0, 4]), [0, 0, 0, 4]);
    }

    #[test]
    fn test_actuate3() {
        let mut board = board::Board::new();
        assert_eq!(board::actuate_row(&[2, 0, 2, 0]), [0, 0, 0, 4]);
    }

    #[test]
    fn test_actuate4() {
        let mut board = board::Board::new();
        assert_eq!(board::actuate_row(&[4, 4, 8, 8]), [0, 0, 8, 16]);
    }

    #[test]
    fn test_rotate1() {
        let mut board = board::Board::new();

        let c1 = [
            [5, 6, 7, 8],
            [0, 1, 2, 3],
            [0, 1, 2, 3],
            [0, 1, 2, 3],
        ];

        let c2 = [
            [0, 0, 0, 5],
            [1, 1, 1, 6],
            [2, 2, 2, 7],
            [3, 3, 3, 8],
        ];

        assert_eq!(board::rotate(&c1), c2);
    }

    #[test]
    fn test_rotate2() {
        let mut board = board::Board::new();

        let c1 = [
            [5, 6, 7, 8],
            [0, 1, 2, 3],
            [0, 1, 2, 3],
            [0, 1, 2, 3],
        ];

        let c2 = [
            [0, 0, 0, 5],
            [1, 1, 1, 6],
            [2, 2, 2, 7],
            [3, 3, 3, 8],
        ];

        assert_eq!(board::rotate(&c1), c2);
    }

    #[test]
    fn test_play() {
        let mut board = board::Board::new();

        let mut changed = true;
        for i in 0..20 {
            if (changed) {
                board.add_random_cell();
                changed = board.actuate(board::Direction::Left);
            }
        }
        board.print();
    }
}
