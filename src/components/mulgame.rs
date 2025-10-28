use leptos::prelude::*;
use leptos::html::*;

use crate::model::Game;

use super::board::board;
use super::boardnav::{board_nav, board_next, board_prev};
use crate::front_model::{GameNavState, GameNavPhase, GameNavPhaseTrait, PlayState};


pub fn mul_game() -> impl IntoView {
    let play_state = PlayState::new(Game::test_game());
    let game_nav_state = GameNavState::new();

    let current_board = Signal::derive(move || play_state.game.with(|game| {
        game.boards.get(game_nav_state.current_board_idx.get()).unwrap().clone()
    }));
    let current_guesses = Signal::derive(move || play_state.guesses[game_nav_state.current_board_idx.get()].clone());
    let active_riddle = RwSignal::new("".to_string());
    let classes = Signal::derive(move || { 
        let base_classes = "mul-game pos-rel wh-100".to_string();
        match game_nav_state.phase.get() {
            GameNavPhase::TransitioningOut => base_classes + " transition-out",
            GameNavPhase::TransitioningIn => base_classes + " transition-in",
            _ => base_classes
        }
    });

    Effect::new(move || {
        if game_nav_state.is_transitioning() {
            active_riddle.set("".to_string());
        }
    });

    let on_next_click = move |_| {
        game_nav_state.transition_to(game_nav_state.current_board_idx.get() + 1);
    };
    let on_prev_click = move |_| {
        game_nav_state.transition_to(game_nav_state.current_board_idx.get() - 1);
    };

    let show_next = Signal::derive(move || {
        game_nav_state.current_board_idx.get() < play_state.game.with(|game| game.boards.len() - 1)
    });
    let show_prev = Signal::derive(move || {
        game_nav_state.current_board_idx.get() > 0
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
    div().class(classes).style(style).child((
        board_nav(game_nav_state, play_state),
        board(current_board, current_guesses, active_riddle),
        board_prev(on_prev_click, show_prev),
        board_next(on_next_click, show_next),
    ))
}
