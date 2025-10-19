use leptos::prelude::*;
use leptos::html::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

fn App() -> impl IntoView {
    (
        h1().child("Hello, world!"),
        p().child("This is a paragraph."),
    )
}
