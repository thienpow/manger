use sycamore::prelude::*;

#[component]
pub fn _Copyright<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,
        footer(class="info") {
            p { "" }
            p { "No Copyright." }
            p {
                "Created by " a(href="https://github.com/thienpow", target="_blank") { "thienpow" }
            }
            p { "Part of Kingdom of God" }
        }
    }
}
