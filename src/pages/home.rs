use sycamore::prelude::*;
use web_sys::{console, Event};


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
        
        div(class="content-wrapper") {

            div(class="content-section") {
                
                div(class="content-section-title") {
                    "CHOOSE A BIBLE"
                }
                div(class="apps-card") {
                    div(class="app-card") {
                        span() {"Bible image here"}
                        div(class="app-card__subtext") {"Please choose a preferred Bible language."}
                        div(class="app-card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"中文简体"}
                            div(class="menu")
                        }
                    }
                }
            }
            /*
            div(class="content-wrapper-header") {
                div(class="content-wrapper-context") {
                    h3(class="img-content") {
                        span(dangerously_set_inner_html=HOME_CONTENT_HEADER_SVG)
                        "Adobe Stock"
                        
                    }
                    div(class="content-text") {
                        "Grab yourself 10 free images from Adobe Stock in a 30-day free trial plan and find perfect image, that will help you with your new project."
                    }
                    button(class="content-button") {
                        "Start free trial"
                    }
                }
                img(class="content-wrapper-img", src="https://assets.codepen.io/3364143/glass.png", alt="")
            }
            */
            
        }
    }
}
