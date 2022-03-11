use sycamore::prelude::*;
use crate::{components::footer, store::{AppState}, pages::profile::{preference::Preference, account::Account}};


#[component]
pub fn Profile<G: Html>(ctx: ScopeRef) -> View<G> {

    let _app_state = ctx.use_context::<AppState>();
    
    view! { ctx,

        div(class="wrapper") {
            div(class="main-container") {

                div(class="content-wrapper") {
                    Account {}
                    Preference {}
                    footer::ContentFooter()
                }
            }
        }
        
    }
}
