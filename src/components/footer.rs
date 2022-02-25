use sycamore::prelude::*;

#[component]
pub fn ContentFooter<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,
        footer(class="info") {
            br()br()br()br()br()br()br()br()
        }
    }
}
