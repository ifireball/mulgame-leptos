use leptos::prelude::*;

use super::board::Board;
use super::boardnav::{BoardNav, BoardNext, BoardPrev};

pub fn MulGame() -> impl IntoView {
    (
        BoardNav(),
        Board(),
        BoardPrev(),
        BoardNext(),
    )
}
