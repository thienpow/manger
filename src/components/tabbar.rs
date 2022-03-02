
use sycamore::prelude::*;


use crate::AppRoutes;
use crate::context::{CurrentRoute};

#[component]
pub fn TabBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    fn get_tabbar_style(current: &AppRoutes) -> String {
        match current {
            AppRoutes::Chat(_) => "display: none;".to_string(), 
            _ => "".to_string()
        }
    }

    view! { ctx,

        div(class="tab-bar", style=get_tabbar_style(&*current_route.get())) {
            div(class="tab-bar-wrapper") {
                div(class="tab-bar-item") {
                    a(aria-label="Manger Home Page", href="/"
                    ) { i(class="gg-home") }
                }
                div(class="tab-bar-item")  {
                    a(aria-label="Bible Study Page", href="/bible"
                    )  { i(class="gg-readme") }
                }
                div(class="tab-bar-item") {
                    a(aria-label="Love in Action", href="/love"
                    ) { i(class="gg-add-r") }
                }
                div(class="tab-bar-item") {
                    a(aria-label="Community Page", href="/community"
                    )  { i(class="gg-smile") }
                }
                div(class="tab-bar-item") {
                    a(aria-label="Profile Page", href="/profile"
                    )  { i(class="gg-profile") }
                }
            }
            
        }
    }

}
