

use sycamore::prelude::*;

use crate::{svg::{BACK_SVG, VIDEO_SVG, PHONE_SVG}, store::AppState};

#[component]
pub fn NavBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let _app_state = ctx.use_context::<AppState>();

    view! { ctx,

        nav(class="navbar") {
            div(class="navbar-menu-left") {
                a(aria-label="Community Page", href="/community",  dangerously_set_inner_html=BACK_SVG) 
            }
            div(class="menu-gap-shrink")
            div(class="navbar-menu") {
                a(href="#") {"@contact_name"} 
                //(*app_state.inner_height.get())
            }
            
            div(class="menu-gap")
            div(class="navbar-menu-right") {
                span(
                    //on:click=toggle,
                    dangerously_set_inner_html=VIDEO_SVG,
                )
                div(class="icon-gap")
                span(
                    //on:click=toggle,
                    dangerously_set_inner_html=PHONE_SVG,
                )
            }
        }
    }
}