
use sycamore::prelude::*;


use crate::route::AppRoutes;
use crate::store::{CurrentRoute};

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

        footer(class="tab-bar", style=get_tabbar_style(&*current_route.get())) {
            div(class="tab-bar-wrapper") {
                a(aria-label="Manger Home Page", href="/") { 
                    div(class="tab-bar-item") {
                        i(class="gg-home") 
                    }
                }
                a(aria-label="Bible Study Page", href="/bible") {
                    div(class="tab-bar-item")  {
                        i(class="gg-readme") 
                    }
                }
                a(aria-label="Love in Action", href="/love") {
                    div(class="tab-bar-mid-item") {
                        i(class="btn-cross") 
                    }
                }
                a(aria-label="Community Page", href="/community")  { 
                    div(class="tab-bar-item") {
                        i(class="gg-smile") 
                    }
                }
                a(aria-label="Profile Page", href="/profile")  { 
                    div(class="tab-bar-item") {
                        i(class="gg-profile") 
                    }
                }
            }
            
        }
    }

}
