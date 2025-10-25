use leptos::prelude::*;
use leptos::html::*;
use leptos::ev;
use std::fmt;

use crate::model::{Board, board_index_to_number, row_col_to_board_index};

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Number(u8),
    Riddle {
        riddle_index: u8,
        possible_answers: [u8; 4],
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Cell::Number(number) = self {
            return write!(f, "{}", number);
        } else {
            return write!(f, "");
        }
    }
}   

pub fn board(
    board: Signal<Board>,
    guesses: Signal<RwSignal<[Option<u8>; 4]>>,
) -> impl IntoView {
    let cells: [RwSignal<Cell>; 100] = (0..100).map(|_| RwSignal::new(Cell::Empty)).collect::<Vec<_>>().try_into().unwrap();

    Effect::new(move || {
        let board = board.get();

        for (idx, cell) in cells.iter().enumerate() {
            cell.set(Cell::Number(board_index_to_number(idx as u8)));
        }

        for (riddle_index, riddle) in board.riddles.iter().enumerate() {
            cells[riddle.board_index as usize].set(Cell::Riddle { 
                riddle_index: riddle_index as u8, 
                possible_answers: riddle.possible_answers 
            });
        }
    });

    table().dir("ltr").class("game-board").style("grid-area: board;").child(
        (0..10).map(|row| {
            tr().child(
                (0..10).map(|col| {
                    board_cell(cells[row_col_to_board_index(row, col) as usize].into(), guesses)
                }).collect::<Vec<_>>().into_view()
            ).into_view()
        }).collect::<Vec<_>>().into_view()
    )
}

fn board_cell(cell: Signal<Cell>, guesses: Signal<RwSignal<[Option<u8>; 4]>>) -> impl IntoView {
    return move || { 
        if let Cell::Riddle { riddle_index, possible_answers } = cell.get() {
            // Can extract guesses here because this is a derived signal that 
            // is expected to be re-called when the cell changes on board 
            // navigation, and the wrapping signal of guesses also depends on
            // the current board index and also only changes on navigation
            let guesses = guesses.get();
            let riddle_index = riddle_index as usize;
            return td().class("aljust-center").class("riddle-controls").child((
                label().child((
                    move || guesses.with(|guesses| {
                        guesses[riddle_index].map(|guess| guess.to_string()).unwrap_or("??".to_string())
                    }),
                    input().r#type("radio").name("active-riddle").value(riddle_index.to_string()),
                )),
                possible_answers.iter().enumerate().map(|(index, answer)| {
                    let answer = answer.clone();
                    input().r#type("button").value(answer.to_string()).style(("--index", index.to_string())).on(ev::click, move |_| {
                        guesses.update(|guesses| guesses[riddle_index] = Some(answer));
                    })
                }).collect::<Vec<_>>(),
            )).into_any();
        } else {
            return td().class("aljust-center").child(cell.get().to_string()).into_any();
        }
    }
}
