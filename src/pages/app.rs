use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use crate::{components::{header::Header, tabbar::TabBar}, AppRoutes, context::CurrentRoute};
use crate::pages::{home::home::Home, bible::bible::Bible, chat::chat::Chat, profile::Profile};

use crate::pages::{bible};

#[component]
pub fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    

    let home_page = view! { ctx, 
        Home()
    };
    let bible_page = view! { ctx, 
        Bible()
    };
    let chat_page = view! { ctx, 
        Chat()
    };

    let profile_page = view! { ctx, 
        Profile()
    };

    view! { ctx,
        
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| {

                let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

                view! { ctx,
                    div(class="app") {
                        
                        Header()

                        (match route.get().as_ref() {
                            AppRoutes::Home => {
                                current_route.set(AppRoutes::Home);
                                home_page.clone()
                            },
                            AppRoutes::Bible => {
                                current_route.set(AppRoutes::Bible);
                                bible::util::scroll_to_selected(ctx);
                                bible_page.clone()
                            },
                            AppRoutes::Love => {
                                current_route.set(AppRoutes::Love);
                                view! { ctx,
                                p {"Love in Action"}
                            }},
                            AppRoutes::Community => {
                                current_route.set(AppRoutes::Community);
                                view! { ctx,
                                p {"Community features here (chat,meeting,sharing,prayer,help)"}
                            }},
                            AppRoutes::Chat(cid) => {
                                current_route.set(AppRoutes::Chat(cid.to_string()));
                                chat_page.clone()
                            },
                            AppRoutes::Profile => {
                                current_route.set(AppRoutes::Profile);
                                profile_page.clone()
                            },
                            AppRoutes::NotFound => {
                                current_route.set(AppRoutes::NotFound);
                                view! { ctx,
                                p { "404 Not Found" }
                            }},
                        })

                        TabBar()
                    }
                    
                }
            }
        }
        
        
    }
}