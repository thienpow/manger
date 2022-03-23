use sycamore::prelude::*;
use web_sys::Event;

pub struct SearchBar {
    pub focus: Box<dyn Fn(Event)>,
    pub blur: Box<dyn Fn(Event)>
}

#[component]
pub fn SearchBar<G: Html>(cx: Scope, props: SearchBar) -> View<G> {

    view! { cx,
        div(class="search-bar") {
            input(type="text", placeholder="Search", on:focus=props.focus, on:blur=props.blur)
        }

    }
}
