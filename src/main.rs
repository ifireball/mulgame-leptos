use leptos::prelude::*;

mod components;

use components::MulGame;

fn main() {
    leptos::mount::mount_to_body(App);
}

fn App() -> impl IntoView {
    (
        MulGame(),
    )
}
