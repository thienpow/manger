use sycamore::prelude::*;


use crate::AppRoutes;
use crate::context::{CurrentRoute};
use crate::svg::{HOME_SVG, BIBLE_SVG, CHAT_SVG, PROFILE_SVG};

#[component]
pub fn TabBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    view! { ctx,
        
        div(class="tab-bar") {
            ul {
                li() {
                    a(class=(match *current_route.get() {AppRoutes::Home => "tab-bar-button is-active", _ => "tab-bar-button"}), 
                        aria-label="Manger Home Page",
                        href="/",
                        dangerously_set_inner_html=HOME_SVG
                    ) 
                }
                li()  {
                    a(class=(match *current_route.get() {AppRoutes::BibleStudy => "tab-bar-button is-active", _ => "tab-bar-button"}), 
                        aria-label="Bible Study Page",
                        href="/bible",
                        dangerously_set_inner_html=BIBLE_SVG
                    ) 
                }
                li() {
                    a(class=(match *current_route.get() {AppRoutes::Love => "tab-bar-button is-active", _ => "tab-bar-button"}), 
                        aria-label="Love in Action",
                        href="/love",
                    ) {
                        img(class="love", loading="lazy",
                        src="assets/img/love.webp", 
                        alt="Love in action")
                    }
                }
                
                li() {
                    a(class=(match *current_route.get() {AppRoutes::Community => "tab-bar-button is-active", _ => "tab-bar-button"}), 
                        aria-label="Community Page",
                        href="/community",
                        dangerously_set_inner_html=CHAT_SVG
                    ) 
                }
                li() {
                    a(class=(match *current_route.get() {AppRoutes::Profile => "tab-bar-button is-active", _ => "tab-bar-button"}), 
                        aria-label="Profile Page",
                        href="/profile",
                        dangerously_set_inner_html=PROFILE_SVG
                    ) 
                }
                
            }
        }
    }
}
