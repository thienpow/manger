

use sycamore::prelude::*;

use crate::AppRoutes;
use crate::context::{CurrentRoute};
use crate::svg::{BACK_SVG, VIDEO_SVG, PHONE_SVG};

#[component]
pub fn NavBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    view! { ctx,

        div(class="navbar") {
            div(class="navbar-menu-left") {
                a(aria-label="Community Page", href="/community",  dangerously_set_inner_html=BACK_SVG) 
            }
            div(class="menu-gap-shrink")
            div(class="navbar-menu") {
                a(href="#") {"@contact_name"}
            }
            
            div(class="menu-gap")
            div(class="navbar-menu-right") {
                span(
                    //on:click=toggle,
                    dangerously_set_inner_html=VIDEO_SVG,
                )
                span(
                    //on:click=toggle,
                    dangerously_set_inner_html=PHONE_SVG,
                )
            }
        }
    }
}