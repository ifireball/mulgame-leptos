use leptos::prelude::*;
use leptos::html::*;

use crate::front_model::{GameNavState, PlayState, BoardScore};

const CORRECT_SMALL_CHAR: &str = "👍";
const INCORRECT_SMALL_CHAR: &str = "👎";
const PARTIAL_SMALL_CHAR: &str = "❓";
const EMPTY_SMALL_CHAR: &str = "❓";
const CORRECT_HERO_CHAR: &str = "💯";
const INCORRECT_HERO_CHAR: &str = "💩";
const PARTIAL_HERO_CHAR: &str = "🤔";
const EMPTY_HERO_CHAR: &str = "🤷";


pub fn score_animator(
    current_board_score: Signal<BoardScore>,
) -> impl IntoView {
    let small_char = Signal::derive(move || {
        match current_board_score.get() {
            BoardScore::Correct => CORRECT_SMALL_CHAR,
            BoardScore::Incorrect => INCORRECT_SMALL_CHAR,
            BoardScore::Partial => PARTIAL_SMALL_CHAR,
            BoardScore::Empty => EMPTY_SMALL_CHAR,
        }
    });
    let hero_char = Signal::derive(move || {
        match current_board_score.get() {
            BoardScore::Correct => CORRECT_HERO_CHAR,
            BoardScore::Incorrect => INCORRECT_HERO_CHAR,
            BoardScore::Partial => PARTIAL_HERO_CHAR,
            BoardScore::Empty => EMPTY_HERO_CHAR,
        }
    });
    
    div().class("score-animator").child((
        (1..=98).map(|_| {
            div().class("cell").child(small_char)
        }).collect::<Vec<_>>(),
        div().class("hero").child(hero_char)
    ))
}
