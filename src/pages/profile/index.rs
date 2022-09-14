use sycamore::prelude::*;
use crate::{components::footer, store::{AppState}, pages::profile::{preference::Preference, account::Account}};


#[component]
pub fn Profile<G: Html>(cx: Scope) -> View<G> {

    let _app_state = use_context::<AppState>(cx);
    
    view! { cx,

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
