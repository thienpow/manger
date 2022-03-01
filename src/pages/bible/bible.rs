use sycamore::prelude::*;
use crate::pages::bible::toc::TOC;


#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {
                TOC()
            }

        }
         
    }
}
