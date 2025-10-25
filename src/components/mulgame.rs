use leptos::prelude::*;
use leptos::html::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;

use crate::model::Game;

use super::board::board;
use super::boardnav::{board_nav, board_next, board_prev};

const BOARD_TRANSITION_DURATION: u32 = 1500;

fn create_transition_task(
    current_board_idx: RwSignal<usize>, 
    transition_class: RwSignal<Option<&'static str>>, 
    new_board_idx: usize
) -> impl std::future::Future<Output = ()> + 'static {
    async move {
        transition_class.set(Some("transition-out"));
        TimeoutFuture::new(BOARD_TRANSITION_DURATION).await;
        current_board_idx.set(new_board_idx);
        transition_class.set(Some("transition-in"));
        TimeoutFuture::new(BOARD_TRANSITION_DURATION).await;
        transition_class.set(None);
    }
}

pub fn mul_game() -> impl IntoView {
    let game = RwSignal::new(Game::test_game());
    let guesses: [RwSignal<[Option<u8>; 4]>; 10] = (0..10).map(|_| RwSignal::new([None; 4])).collect::<Vec<_>>().try_into().unwrap();

    let current_board_idx = RwSignal::new(0);
    let current_board = Signal::derive(move || game.with(|game| {
        game.boards.get(current_board_idx.get()).unwrap().clone()
    }));
    let current_guesses = Signal::derive(move || guesses[current_board_idx.get()].clone());
    let transition_class = RwSignal::new(None);
    let classes = Signal::derive(move || { "mul-game pos-rel wh-100 ".to_string() + transition_class.get().unwrap_or("") });

    let on_next_click = move |_| {
        spawn_local(create_transition_task(
            current_board_idx,
            transition_class,
            current_board_idx.get() + 1
        ));
    };
    
    let on_prev_click = move |_| {
        spawn_local(create_transition_task(
            current_board_idx,
            transition_class,
            current_board_idx.get() - 1
        ));
    };

    let show_next = Signal::derive(move || {
        current_board_idx.get() < game.with(|game| game.boards.len() - 1)
    });
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
    div().class(classes).style(style).child((
        board_nav(),
        board(current_board, current_guesses),
        board_prev(on_prev_click, show_prev),
        board_next(on_next_click, show_next),
    ))
}
