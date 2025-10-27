use leptos::prelude::*;

mod model;
mod front_model;
mod components;

use components::mul_game;

fn main() {
    leptos::mount::mount_to_body(app);
}

fn app() -> impl IntoView {
    (
        mul_game(),
    )
}
