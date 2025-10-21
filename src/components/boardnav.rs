use leptos::prelude::*;
use leptos::html::*;

pub fn BoardNav() -> impl IntoView {
    let style = "
        grid-area: nav;
        display: flex;
        flex-direction: row;
        justify-content: space-around;
    ";

    nav().class("board-nav pos-rel wh-100").style(style).child(
        (1..=10).map(|i| {
            button().class("aljust-center").child(format!("{}", i))
        }).collect::<Vec<_>>().into_view()
    )
}

pub fn BoardNext() -> impl IntoView {
    button().class("aljust-center").style("grid-area: next;").child("הבא")
}

pub fn BoardPrev() -> impl IntoView {
    button().class("aljust-center").style("grid-area: prev;").child("הקודם")
}
