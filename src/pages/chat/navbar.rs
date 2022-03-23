

use sycamore::prelude::*;

use crate::svg::{BACK_SVG, VIDEO_SVG, PHONE_SVG};

#[component]
pub fn NavBar<G: Html>(cx: Scope) -> View<G> {

/*
    let app_state = use_context::<AppState>(cx);

    let mut  fragement: Vec<View<G>> = Vec::new();
    let cool_button: G = node! { ctx, button { "The coolest 😎" } };
    
    fragement.push(View::new_node(cool_button));

    let nav_item =  View::new_fragment(fragement);
 */

    view! { cx,

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