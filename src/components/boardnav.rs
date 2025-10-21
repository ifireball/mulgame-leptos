use leptos::prelude::*;
use leptos::html::*;

pub fn BoardNav() -> impl IntoView {
    nav().class("board-nav").child(
        (1..=10).map(|i| {
            button().child(format!("{}", i))
        }).collect::<Vec<_>>().into_view()
    )
}

pub fn BoardNext() -> impl IntoView {
    button().child("הבא")
}

pub fn BoardPrev() -> impl IntoView {
    button().child("הקודם")
}
