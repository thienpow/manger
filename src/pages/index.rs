
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::ScopeSpawnLocal;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use crate::components::{header::Header, tabbar::TabBar};
use crate::pages::{home, bible, chat, profile};
use crate::route::AppRoutes; 
use crate::store::{CurrentRoute, AppState};

#[component]
pub fn Index<G: Html>(ctx: Scope) -> View<G> {
    

    view! { ctx,
        
        Router {
            integration: HistoryIntegration::new(),
            view: |ctx, route: &ReadSignal<AppRoutes>| {

                let page_index = ctx.create_signal(0);

                let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();


                let app_state = ctx.use_context::<AppState>();
    
                let app_content_ref = ctx.create_node_ref();
                let slide_in = move |i: i32| {
                    page_index.set(i);
                    if *app_state.inner_width.get() <= 738.0 {
                        ctx.spawn_local(async move {
                            TimeoutFuture::new(120).await;
                            let content = app_content_ref.get::<DomNode>();
                            content.set_attribute("style", "transform: translate(0, 0); transition: transform 777ms ease-in-out;");
                        });
                    }
                    
                };
                
                let reset_slide = move |i: i32| {
                    if *app_state.inner_width.get() <= 738.0 {
                        let content = app_content_ref.get::<DomNode>();

                        if i == *page_index.get() {

                        } else if i < *page_index.get() {
                            content.set_attribute("style", "transform: translateX(-88px);-webkit-animation: fadein 0.5s;animation: fadein 0.5s;");
                        } else if i > *page_index.get() {
                            content.set_attribute("style", "transform: translateX(88px);-webkit-animation: fadein 0.5s;animation: fadein 0.5s;");
                        }
                        
                    }
                    
                };


                let home_page = view! { ctx, 
                    home::index::Home()
                };
                let bible_page = view! { ctx, 
                    bible::index::Bible()
                };
                let chat_page = view! { ctx, 
                    chat::index::Chat()
                };
                let profile_page = view! { ctx, 
                    profile::profile::Profile()
                };

                view! { ctx,
                    div(class="app") {
                        
                        Header()

                        div(ref=app_content_ref, class="app-content") {(
                            {
                                
                                match route.get().as_ref() {
                                    AppRoutes::Home => {
                                        reset_slide(0);
                                        current_route.set(AppRoutes::Home);
                                        slide_in(0);
                                        home_page.clone()
                                    },
                                    AppRoutes::Bible => {
                                        reset_slide(1);
                                        current_route.set(AppRoutes::Bible);
                                        bible::util::scroll_to_selected_book(ctx, 60);
                                        bible::util::scroll_to_selected_chapter(ctx, 1000);
                                        slide_in(1);
                                        bible_page.clone()
                                        
                                    },
                                    AppRoutes::Love => {
                                        reset_slide(2);
                                        current_route.set(AppRoutes::Love);
                                        slide_in(2);
                                        view! { ctx,
                                        p {"Love in Action"}
                                    }},
                                    AppRoutes::Community => {
                                        reset_slide(3);
                                        current_route.set(AppRoutes::Community);
                                        slide_in(3);
                                        view! { ctx,
                                        p {"Community features here (chat,meeting,sharing,prayer,help)"}
                                    }},
                                    AppRoutes::Chat(cid) => {
                                        reset_slide(4);
                                        current_route.set(AppRoutes::Chat(cid.to_string()));
                                        slide_in(4);
                                        chat_page.clone()
                                    },
                                    AppRoutes::Profile => {
                                        reset_slide(5);
                                        current_route.set(AppRoutes::Profile);
                                        slide_in(5);
                                        profile_page.clone()
                                    },
                                    AppRoutes::NotFound => {
                                        reset_slide(6);
                                        current_route.set(AppRoutes::NotFound);
                                        slide_in(6);
                                        view! { ctx,
                                        p { "404 Not Found" }
                                    }},
                                }
                            }
                        )}

                        TabBar()
                    }
                    
                }
            }
        }
        
        
    }
}