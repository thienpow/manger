use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use crate::components::{header::Header, tabbar::TabBar};
use crate::pages::{bible, chat, home, profile};
use crate::route::AppRoutes;
use crate::store::{AppState, CurrentRoute};

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,

        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<AppRoutes>| {

                let page_index = create_signal(cx, 0);

                let CurrentRoute(current_route) = use_context::<CurrentRoute>(cx);

                let app_state = use_context::<AppState>(cx);

                let app_content_ref = create_node_ref(cx);
                let slide_in = move |i: i32| {
                    if *page_index.get() == i {
                        return;
                    }

                    page_index.set(i);
                    if *app_state.inner_width.get() <= 738.0 {
                        spawn_local_scoped(cx, async move {
                            TimeoutFuture::new(120).await;
                            let content = app_content_ref.get::<DomNode>();
                            content.set_attribute("style", "transform: translate(0, 0); transition: transform 288ms ease-in-out;");
                        });
                    }

                };

                let reset_slide = move |i: i32| {
                    if *app_state.inner_width.get() <= 738.0 {
                        let content = app_content_ref.get::<DomNode>();

                        let pi = *page_index.get();
                        let mut x = "0";
                        if i == pi {
                        } else if i < pi {
                            x = "-38"
                        } else if i > pi {
                            x = "38"
                        }

                        let style = format!("-webkit-animation:fadein 0.288s;animation:fadein 0.288s; transform:translateX({}px);", x);
                        content.set_attribute("style", &style);
                    }

                };


                let home_page = view! { cx,
                    home::index::Home()
                };
                let bible_page = view! { cx,
                    bible::index::Bible()
                };
                let chat_page = view! { cx,
                    chat::index::Chat()
                };
                let profile_page = view! { cx,
                    profile::index::Profile()
                };

                view! { cx,
                    div(class="app") {

                        Header()

                        div(ref=app_content_ref, class="app-content") {(
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
                                        bible::util::scroll_to_selected_book(cx, 60);
                                        bible::util::scroll_to_selected_chapter(cx, 1000);
                                        slide_in(1);
                                        bible_page.clone()
                                    },
                                    AppRoutes::Love => {
                                        reset_slide(2);
                                        current_route.set(AppRoutes::Love);
                                        slide_in(2);
                                        view! { cx,
                                        p {"Love in Action"}
                                    }},
                                    AppRoutes::Community => {
                                        reset_slide(3);
                                        current_route.set(AppRoutes::Community);
                                        slide_in(3);
                                        view! { cx,
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
                                        view! { cx,
                                        p { "404 Not Found" }
                                    }},
                                }
                        )}

                        TabBar()
                    }

                }
            }
        }


    }
}
