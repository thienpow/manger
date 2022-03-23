

use sycamore::prelude::*;

use crate::svg::{THUMBS_UP_SVG};

#[component]
pub fn Footer<G: Html>(ctx: Scope) -> View<G> {
    
    view! { ctx,

        div(class="chat-footer") {
            i(class="icon-image optinal-button")
            
            i(class="icon-attachment")

            input(type="text", placeholder="Type something here...")

            i(class="icon-smile")

            span(class="optinal-button", dangerously_set_inner_html=THUMBS_UP_SVG)
        }
    }
}