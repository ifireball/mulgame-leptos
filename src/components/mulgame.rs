use leptos::prelude::*;
use leptos::html::*;

use super::board::Board;
use super::boardnav::{BoardNav, BoardNext, BoardPrev};

pub fn MulGame() -> impl IntoView {
    let style = "
        display: grid;
        grid-template-columns: 1fr 75vh 1fr;
        grid-template-rows: 1fr 75vh 1fr;
        grid-template-areas:
            \"...  nav   ...\"
            \"next board prev\"
            \"...  ...   ...\";
    ";
    div().class("mul-game pos-rel wh-100").style(style).child((
        BoardNav(),
        Board(),
        BoardPrev(),
        BoardNext(),
    ))
}
