use sycamore::prelude::*;

#[component]
pub fn ContentFooter<G: Html>(cx: Scope) -> View<G> {

    view! { cx,
        footer(class="info") {
            br()br()br()br()br()br()br()br()
        }
    }
}
