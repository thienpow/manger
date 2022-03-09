use sycamore::prelude::*;
use crate::components::contactbar::ContactBar;
use crate::pages::chat::{navbar::NavBar, content::Content, footer::Footer};

#[component]
pub fn Chat<G: Html>(ctx: ScopeRef) -> View<G> {
    
    view! { ctx,

        div(class="wrapper") {

            article(class="main-container") {

                div(class="chat-wrapper") { 
                    NavBar {}
                    Content {}
                }
                
            }

            ContactBar {}
            Footer {}
        }

        
    }
}
