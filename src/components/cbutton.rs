use sycamore::prelude::*;
use web_sys::Event;

pub struct CButton {
    pub label: String,
    pub click: Box<dyn Fn(Event)>,
}

#[component]
pub fn CButton<G: Html>(ctx: ScopeRef, props: CButton) -> View<G> {
    view! { ctx,
        button(class="cbutton status-button", on:click=props.click) {(props.label)}
    }
}
