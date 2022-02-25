use sycamore::prelude::*;

use crate::pages::{chat::ChatArea};

#[component]
pub fn People<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,

        ChatArea()
    }
}
