use leptos::prelude::*;
use leptos::html::*;

pub fn Board() -> impl IntoView {
    table().class("game-board").child(
        (1..10).map(|row| {
            tr().child(
                (1..10).map(|col| {
                    td().child(format!("{}", row * col))
                }).collect::<Vec<_>>().into_view()
            ).into_view()
        }).collect::<Vec<_>>().into_view()
    )
}
