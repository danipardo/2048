mod board;

pub fn main() {}


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
