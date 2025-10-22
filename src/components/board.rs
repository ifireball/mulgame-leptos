use leptos::prelude::*;
use leptos::html::*;

use crate::model::{Board, Cell, board_index_to_number, row_col_to_board_index};

pub fn Board(
    board: Signal<Board>
) -> impl IntoView {
    let cells: [RwSignal<Cell>; 100] = (0..100).map(|_| RwSignal::new(Cell::Empty)).collect::<Vec<_>>().try_into().unwrap();

    Effect::new(move || {
        let board = board.get();

        for (idx, cell) in cells.iter().enumerate() {
            cell.set(Cell::Number(board_index_to_number(idx as u8)));
        }

        for riddle in board.riddles {
            cells[riddle.board_index as usize].set(Cell::Riddle { possible_answers: riddle.possible_answers, guess: None });
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
    let text = move || { cell.get().to_string() };
    return text
}
