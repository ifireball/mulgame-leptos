use leptos::prelude::*;

use crate::model::Game;

#[derive(Clone, Copy)]
pub struct PlayState {
    pub game: RwSignal<Game>,
    pub guesses: [RwSignal<[Option<u8>; 4]>; 10],
    pub submissions: [RwSignal<bool>; 10],
}

#[derive(Clone, Copy)]
pub enum BoardScore {
    Correct,
    Incorrect,
    Partial,
    Empty,
}

impl PlayState {
    pub fn new(game: Game) -> Self {
        Self { 
            game: RwSignal::new(game), 
            guesses: (0..10).map(|_| RwSignal::new([None; 4])).collect::<Vec<_>>().try_into().unwrap(), 
            submissions: (0..10).map(|_| RwSignal::new(false)).collect::<Vec<_>>().try_into().unwrap(),
        }
    }
    pub fn get_board_score(&self, board_idx: usize) -> BoardScore {
        let board = self.game.with(|game| game.boards[board_idx]);
        let guesses = self.guesses[board_idx].get();

        let mut correct_guesses = 0;
        let mut incorrect_guesses = 0;
        for (guess, riddle) in guesses.iter().zip(board.riddles.iter()) {
            if let Some(guess) = guess {
                if *guess == riddle.answer() { correct_guesses += 1 } else { incorrect_guesses += 1 }
            }
        }
        if incorrect_guesses > 0 {
            return BoardScore::Incorrect;
        }
        if correct_guesses == 0 {
            return BoardScore::Empty;
        }
        if correct_guesses < 4 {
            return BoardScore::Partial;
        }
        return BoardScore::Correct;
    }
    pub fn derive_board_score(&self, board_idx: usize) -> Signal<BoardScore> {
        let play_state = self.clone();
        Signal::derive(move || play_state.get_board_score(board_idx))
    }
}
