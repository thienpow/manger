use sycamore::prelude::*;
use web_sys::{console, Event};

use crate::{components::{contactbar::ContactBar}, pages::home::sidebar::SidebarLeft};


#[component]
pub fn Home<G: Html>(ctx: ScopeRef) -> View<G> {

    let handle_click_bible = move |_e: Event| {
        console::log_1(&format!("clicked").as_str().into());
    };
    
    view! { ctx,

        /*
        div(class="main-header") {
            div(class="header-menu") {
                a(class="main-header-link is-active", href="#") { "Desktop" }
                a(class="main-header-link", href="#") { "Mobile" }
                a(class="main-header-link", href="#") { "Web" }
            }
        }
        */
        
        div(class="wrapper") {

            SidebarLeft()

            div(class="main-container") {

                div(class="content-wrapper") {
                    div(class="content-section") {
                        div(class="content-section-title") {
                            "CHOOSE A BIBLE"
                        }
                        div(class="card-wrapper") {
                            div(class="card") {
                                span() {"Bible image here"}
                                div(class="card__subtext") {"Please choose a preferred Bible language."}
                                div(class="card-buttons") {
                                    button(class="cbutton status-button", on:click=handle_click_bible) {"中文简体"}
                                    div(class="menu")
                                }
                            }
                        }
                    }
                }

            }

            ContactBar()
        }

                            
    }
}
