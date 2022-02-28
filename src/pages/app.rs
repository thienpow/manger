use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use crate::{components::{header::Header, sidebar::SidebarLeft, contactbar::ContactBar, tabbar::TabBar}, AppRoutes, context::CurrentRoute};
use crate::pages::{home::Home, profile::Profile, chat::chat::Chat};

#[component]
pub fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    
    view! { ctx,
        
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| {

                let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

                view! { ctx,
                    div(class="app") {
                        Header()

                        div(class="wrapper") {

                            SidebarLeft()

                            div(class="main-container") {

                                (match route.get().as_ref() {
                                    AppRoutes::Home => {
                                        current_route.set(AppRoutes::Home);
                                        view! { ctx, 
                                            Home()
                                        }
                                    },
                                    AppRoutes::BibleStudy => {
                                        current_route.set(AppRoutes::BibleStudy);
                                        view! { ctx,
                                        p {"Bible Study feature here(bibles,+tags,blog/vlog,books)"}
                                    }},
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
                                        view! { ctx,
                                            Chat()
                                        }
                                    },
                                    AppRoutes::Profile => {
                                        current_route.set(AppRoutes::Profile);
                                        view! { ctx, 
                                            Profile()
                                        }
                                    },
                                    AppRoutes::NotFound => {
                                        current_route.set(AppRoutes::NotFound);
                                        view! { ctx,
                                        p { "404 Not Found" }
                                    }},
                                })

                            }

                            ContactBar()
                            
                        }
                        
                        TabBar()
                    }
                    
                }
            }
        }
        
        
    }
}