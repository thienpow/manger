use sycamore::prelude::*;

#[component]
pub fn ContentFooter<G: Html>(ctx: Scope) -> View<G> {

    view! { ctx,
        footer(class="info") {
            br()br()br()br()br()br()br()br()
        }
    }
}
