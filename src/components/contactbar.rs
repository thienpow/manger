use sycamore::prelude::*;

use crate::{store::CurrentRoute, route::AppRoutes};

#[component]
pub fn ContactBar<G: Html>(ctx: ScopeRef) -> View<G> {

    let CurrentRoute(current_route) = ctx.use_context::<CurrentRoute>();

    
    view! { ctx,
        div(class="side-bar-contact") {
            /* Cell Groups */
            div(class="side-wrapper") {

                div(class="side-title-contact") {
                    "CELL GROUP"
                }
                div(class="side-menu") {

                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q123" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q123", alt="Sacha Griffin") {
                        div(class="avatar") {
                            img(src="/assets/img/avatar_1.webp", alt="Sacha Griffin", loading="lazy")
                            span(class="status online")
                        }
                        span(class="info") {
                            div(class="name") { "John Study Group" } br()
                            div(class="status-msg") { "Every Wednesday night 8pm" }
                            span(class="unread-messages") { "11" }
                        }
                    }


                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q124" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q124", alt="Sacha Griffin") {
                        span(class="avatar") {
                            img(src="/assets/img/avatar_2.webp", alt="Debby Jones", loading="lazy")
                            span(class="status online")
                        }
                        span(class="info") {
                            div(class="name") { "Debby Group" } br()
                            div(class="status-msg") { "Saturday Morning Gathering" }
                            span(class="last-login") { "Just now" }
                        }
                    }


                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q125" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q125", alt="Sacha Griffin") {
                        div(class="avatar") {
                            img(src="/assets/img/avatar_3.webp", alt="Sarah White", loading="lazy")
                            span(class="status busy")
                        }
                        span(class="info") {
                            div(class="name") { "Sarah & Paul Family" } br()
                            div(class="status-msg") { "Live now" }
                            span(class="last-login") { "2 min" }
                        }
                    }

                }
                
            }

            /* Bro & Sis */
            div(class="side-wrapper") {

                div(class="side-title-contact") {
                    "BRO & SIS"
                }
                div(class="side-menu") {
                    

                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q126" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q126", alt="Sacha Griffin") {
                        div(class="avatar") {
                            img(src="/assets/img/avatar_1.webp", alt="Sacha Griffin", loading="lazy")
                            span(class="status online")
                        }
                        span(class="info") {
                            div(class="name") { "Sacha Griffin" } br()
                            div(class="status-msg") { "Super deep status message blah blah" }
                            span(class="unread-messages") { "11" }
                        }
                    }


                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q127" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q127", alt="Sacha Griffin") {
                        span(class="avatar") {
                            img(src="/assets/img/avatar_2.webp", alt="Debby Jones", loading="lazy")
                            span(class="status away")
                        }
                        span(class="info") {
                            div(class="name") { "Debby Jones" } br()
                            div(class="status-msg") { "New day, fresh start, fresh eadaf" }
                            span(class="last-login") { "Just now" }
                        }
                    }


                    a(class=(match &*current_route.get() {
                        AppRoutes::Chat(cid)=> {if cid == "Q128" {"person active"} else {"person"}},
                        _ => "person"
                    }), href="/chat/Q128", alt="Sacha Griffin") {
                        div(class="avatar") {
                            img(src="/assets/img/avatar_3.webp", alt="Sarah White", loading="lazy")
                            span(class="status busy")
                        }
                        span(class="info") {
                            div(class="name") { "Sarah White" } br()
                            div(class="status-msg") { "Life becomes more peaceful when" }
                            span(class="last-login") { "2 min" }
                        }
                    }


                }
                
            }


        }
        
    }
}
