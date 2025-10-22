fn board_index_to_row_col(board_index: u8) -> (u8, u8) {
    (board_index / 10, board_index % 10)
}

fn row_col_to_board_index(row: u8, col: u8) -> u8 {
    row * 10 + col
}

fn board_index_to_number(board_index: u8) -> u8 {
    let (row, col) = board_index_to_row_col(board_index);
    return row_col_to_number(row, col);
}

fn row_col_to_number(row: u8, col: u8) -> u8 {
    return (row + 1) * (col + 1);
}

#[derive(Clone, Copy)]
pub struct Riddle {
    pub board_index: u8,
    pub possible_answers: [u8; 4]
}

impl Riddle {
    pub fn answer(&self) -> u8 {
        return board_index_to_number(self.board_index);
    }
    pub fn possible_answers(&self) -> [u8; 4] {
        return self.possible_answers;
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub riddles: [Riddle; 4]
}

pub struct Game {
    pub boards: [Board; 10]
}

enum Difficulty {
    MissingCell,
    MissingRowCol,
    MissingAll
}

pub enum Cell {
    Empty,
    Number(u8),
    Riddle {
        possible_answers: [u8; 4],
        guess: Option<u8>
    }
}

impl Game {
    pub fn test_game() -> Game {
        let board1 = Board {riddles: [
            Riddle {board_index: 11, possible_answers: [4, 2, 9, 8]}, // answer 4
            Riddle {board_index: 42, possible_answers: [10, 35, 15, 16]}, // answer 15
            Riddle {board_index: 38, possible_answers: [63, 30, 36, 48]}, // answer 36 
            Riddle {board_index: 87, possible_answers: [28, 58, 8, 72]}, // answer 72
        ]};
        let board2 = Board {riddles: [
            Riddle {board_index: 3, possible_answers: [1, 2, 3, 4]}, // answer 4
            Riddle {board_index: 39, possible_answers: [60, 32, 40, 30]}, // answer 40
            Riddle {board_index: 96, possible_answers: [70, 45, 60, 40]}, // answer 70
            Riddle {board_index: 60, possible_answers: [12, 7, 15, 24]}, // answer 7
        ]};
        let board3 = Board {riddles: [
            Riddle {board_index: 0, possible_answers: [1, 2, 4, 9]}, // answer 1
            Riddle {board_index: 9, possible_answers: [10, 15, 9, 16]}, // answer 10
            Riddle {board_index: 90, possible_answers: [9, 10, 16, 12]}, // answer 10
            Riddle {board_index: 99, possible_answers: [100, 81, 64, 90]}, // answer 100
        ]};
        return Game {boards: [board1, board2, board3, board1, board2, board3, board1, board2, board3, board1]};
    }
}
