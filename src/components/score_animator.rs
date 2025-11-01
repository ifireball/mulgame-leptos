use leptos::prelude::*;
use leptos::html::*;

pub fn score_animator() -> impl IntoView {
    let small_char = "👍";
    
    div().class("score-animator").child(
        (1..=98).map(|i| {
            div().class("cell").child(format!("{}", small_char))
        }).collect::<Vec<_>>()
    )
}
