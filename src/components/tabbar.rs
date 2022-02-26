
use sycamore::prelude::*;


use crate::AppRoutes;
use crate::context::{CurrentRoute};
use crate::svg::{HOME_SVG, BIBLE_SVG, CHAT_SVG, PROFILE_SVG};

#[component]
pub fn TabBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    fn get_a_class(route: &AppRoutes, current: &AppRoutes) -> String {

        let button_active_class: String = "tab-bar-button is-active".to_string();
        let button_empty_class: String = "".to_string();
        match current {
            AppRoutes::Home => match route {
                AppRoutes::Home => button_active_class, 
                _ => button_empty_class
            },
            AppRoutes::BibleStudy => match route {
                AppRoutes::BibleStudy => button_active_class, 
                _ => button_empty_class
            },
            AppRoutes::Love => match route {
                AppRoutes::Love => button_active_class, 
                _ => button_empty_class
            },
            AppRoutes::Community => match route {
                AppRoutes::Community => button_active_class, 
                _ => button_empty_class
            },
            AppRoutes::Profile => match route {
                AppRoutes::Profile => button_active_class, 
                _ => button_empty_class
            },
            _ => "tab-bar-button".to_string()
        }
        
    }

    fn get_tabbar_style(current: &AppRoutes) -> String {
        match current {
            AppRoutes::Chat(_) => "display: none;".to_string(), 
            _ => "".to_string()
        }
    }

    view! { ctx,

        div(class="tab-bar", style=get_tabbar_style(&*current_route.get())) {
            ul {
                li() {
                    a(class=get_a_class(&AppRoutes::Home, &*current_route.get()), aria-label="Manger Home Page", href="/", dangerously_set_inner_html=HOME_SVG) 
                }
                li()  {
                    a(class=get_a_class(&AppRoutes::BibleStudy, &*current_route.get()), aria-label="Bible Study Page", href="/bible", dangerously_set_inner_html=BIBLE_SVG) 
                }
                li() {
                    a(class=get_a_class(&AppRoutes::Love, &*current_route.get()), aria-label="Love in Action", href="/love") {
                        img(class="love", loading="lazy",
                        src="/assets/img/love.webp", 
                        alt="Love in action")
                    }
                }
                li() {
                    a(class=get_a_class(&AppRoutes::Community, &*current_route.get()), aria-label="Community Page", href="/community", dangerously_set_inner_html=CHAT_SVG) 
                }
                li() {
                    a(class=get_a_class(&AppRoutes::Profile, &*current_route.get()), aria-label="Profile Page", href="/profile", dangerously_set_inner_html=PROFILE_SVG) 
                }
                
            }
        }
    }

}
