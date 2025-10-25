use leptos::prelude::*;
use leptos::html::*;
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

pub fn Board(
    board: Signal<Board>
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
                    td().class("aljust-center").child(
                        BoardCell(cells[row_col_to_board_index(row, col) as usize].into())
                    )
                }).collect::<Vec<_>>().into_view()
            ).into_view()
        }).collect::<Vec<_>>().into_view()
    )
}

fn BoardCell(cell: Signal<Cell>) -> impl IntoView {
    return move || { 
        if let Cell::Riddle { riddle_index, possible_answers } = cell.get() {
            return label().class("riddle-controls").child((
                format!("({})", riddle_index),
                input().r#type("radio").name("active-riddle").value(riddle_index.to_string()),
                possible_answers.iter().map(|answer| {
                    input().r#type("button").value(answer.to_string())
                }).collect::<Vec<_>>()
            )).into_any();
        } else {
            return cell.get().to_string().into_any();
        }
    }
}
