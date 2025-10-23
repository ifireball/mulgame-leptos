use std::ops::Deref;

use leptos::prelude::*;
use leptos::html::*;

use crate::model::Game;

use super::board::Board;
use super::boardnav::{BoardNav, BoardNext, BoardPrev};

pub fn MulGame() -> impl IntoView {
    let game = RwSignal::new(Game::test_game());
    let current_board_idx = RwSignal::new(0);
    let current_board = Signal::derive(move || game.with(|game| {
        game.boards.get(current_board_idx.get()).unwrap().clone()
    }));

    let on_next_click = move |_| {
        current_board_idx.set(current_board_idx.get() + 1);
    };
    let show_next = Signal::derive(move || {
        current_board_idx.get() < game.with(|game| game.boards.len() - 1)
    });

    let on_prev_click = move |_| {
        current_board_idx.set(current_board_idx.get() - 1);
    };
    let show_prev = Signal::derive(move || {
        current_board_idx.get() > 0
    });

    let style = "
        display: grid;
        grid-template-columns: 1fr 75vh 1fr;
        grid-template-rows: 1fr 75vh 1fr;
        grid-template-areas:
            \"...  nav   ...\"
            \"prev board next\"
            \"...  ...   ...\";
    ";
    div().class("mul-game pos-rel wh-100").style(style).child((
        BoardNav(),
        Board(current_board),
        BoardPrev(on_prev_click, show_prev),
        BoardNext(on_next_click, show_next),
    ))
}
