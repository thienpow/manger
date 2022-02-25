use sycamore::prelude::*;
use web_sys::{console, Event};

use crate::components::footer;


#[component]
pub fn Profile<G: Html>(ctx: ScopeRef) -> View<G> {

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
                
                div(class="content-section-title") { "PROFILE DATA" }
                div(class="apps-card") {
                    div(class="app-card") {
                        div(style="width:100%; display: flex; justify-content: center;") {
                            img(class="profile-img-big", loading="lazy", 
                                src="/assets/img/avatar_1.webp", 
                                alt="My Profile")
                        }

                        div(class="app-card__subtext") {
                            span(){"Display Name: "}
                        }
                        div(class="app-card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }
                    div(class="app-card") {

                        div(class="app-card__subtext") {
                            span(){"Real Name: "}
                            span(){"Email: "}
                            span(){"Phone: "}
                            div(class="app-card__seperator")
                            span(){"Age: "}
                            span(){"Gender: "}
                            span(){"Race: "}
                            span(){"Job: "}
                        }
                        div(class="app-card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }
                    div(class="app-card") {

                        div(class="app-card__subtext") {
                            span(){"Church Name: "}
                            span(){"Cell Group: "}
                            span(){"Other Info: "}
                            div(class="app-card__seperator")
                            span(){"Address: "}
                            span(){"City: "}
                            span(){"ZipCode: "}
                            span(){"State: "}
                            span(){"Country: "}
                        }
                        div(class="app-card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }
                }

            }

            div(class="content-section") {
                div(class="content-section-title") { "SYSTEM PREFERENCES" }
                div(class="apps-card") {
                    div(class="app-card") {
                        
                        div(class="app-card__subtext") {
                            span(){"Appearance: "}
                            span(){"Accent Color: "}
                            span(){"Background Image: "}
                            span(){"Background Video: "}
                        }
                        div(class="app-card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Update"}
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
            footer::ContentFooter()
        }
    }
}
