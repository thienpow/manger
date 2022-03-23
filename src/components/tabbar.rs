
use sycamore::prelude::*;


use crate::route::AppRoutes;
use crate::store::CurrentRoute;

#[component]
pub fn TabBar<G: Html>(cx: Scope) -> View<G> {

    let CurrentRoute(current_route) = use_context::<CurrentRoute>(cx);

    fn get_tabbar_style(current: &AppRoutes) -> String {
        match current {
            AppRoutes::Chat(_) => "display: none;".to_string(), 
            _ => "".to_string()
        }
    }

    fn get_highlight_style(app_route: AppRoutes, current: &AppRoutes) -> String {
        if &app_route == current {
            "color: var(--highlight-color);".to_string()
        } else {
            "".to_string()
        }
    }

    view! { cx,

        footer(class="tabbar", style=get_tabbar_style(&*current_route.get())) {
            div(class="tabbar-wrapper") {
                a(aria-label="Manger Home Page", href="/") { 
                    div(class="tabbar-item") {
                        i(class="icon-home", style=get_highlight_style(AppRoutes::Home, &*current_route.get())) 
                    }
                }
                a(aria-label="Bible Study Page", href="/bible") {
                    div(class="tabbar-item")  {
                        i(class="icon-readme", style=get_highlight_style(AppRoutes::Bible, &*current_route.get())) 
                    }
                }
                a(aria-label="Love in Action", href="/love") {
                    div(class="tabbar-mid-item") {
                        i(class="icon-cross", style=get_highlight_style(AppRoutes::Love, &*current_route.get())) 
                    }
                }
                a(aria-label="Community Page", href="/community")  { 
                    div(class="tabbar-item") {
                        i(class="icon-smile", style=get_highlight_style(AppRoutes::Community, &*current_route.get())) 
                    }
                }
                a(aria-label="Profile Page", href="/profile")  { 
                    div(class="tabbar-item") {
                        i(class="icon-profile", style=get_highlight_style(AppRoutes::Profile, &*current_route.get())) 
                    }
                }
            }
            
        }
    }

}
