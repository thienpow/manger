

use sycamore::prelude::*;

#[component]
pub fn SafeArea<G: Html>(cx: Scope) -> View<G> {

    view! { cx,
        div(class="safearea")
    }
}
