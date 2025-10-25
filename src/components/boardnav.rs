use leptos::prelude::*;
use leptos::html::*;
use leptos::tachys::html::event::click;
use leptos::tachys::html::event::MouseEvent;

pub fn board_nav() -> impl IntoView {
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

pub fn board_next(on_click: impl FnMut(MouseEvent) + 'static, show: Signal<bool>) -> impl IntoView {
    button().class("aljust-center").style(("grid-area", "next")).child("הבא").on(click, on_click)
    .style(move || if show.get() { ("display", "block") } else { ("display", "none") })
}

pub fn board_prev(on_click: impl FnMut(MouseEvent) + 'static, show: Signal<bool>) -> impl IntoView {
    button().class("aljust-center").style(("grid-area", "prev")).child("הקודם").on(click, on_click)
    .style(move || if show.get() { ("display", "block") } else { ("display", "none") })
}
