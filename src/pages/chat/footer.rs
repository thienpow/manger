

use sycamore::prelude::*;

use crate::svg::{IMAGE_SVG, PAPER_CLIP_SVG, SMILE_SVG, THUMBS_UP_SVG};

#[component]
pub fn Footer<G: Html>(ctx: ScopeRef) -> View<G> {
    
    view! { ctx,

        div(class="chat-footer") {
            span(class="optinal-button", dangerously_set_inner_html=IMAGE_SVG)
            // can use gg-image
            
            span(dangerously_set_inner_html=PAPER_CLIP_SVG)
            // can use gg-attachment

            input(type="text", placeholder="Type something here...")

            span(dangerously_set_inner_html=SMILE_SVG)
            // can use gg-smile

            span(class="optinal-button", dangerously_set_inner_html=THUMBS_UP_SVG)
        }
    }
}