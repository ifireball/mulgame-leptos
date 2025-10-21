use leptos::prelude::*;
use leptos::html::*;

pub fn Board() -> impl IntoView {
    table().dir("ltr").class("game-board").style("grid-area: board;").child(
        (1..=10).map(|row| {
            tr().child(
                (1..=10).map(|col| {
                    td().class("aljust-center").child(format!("{}", row * col))
                }).collect::<Vec<_>>().into_view()
            ).into_view()
        }).collect::<Vec<_>>().into_view()
    )
}
