use sycamore::prelude::*;
use web_sys::{console, Event};

use crate::{components::footer, context::DarkMode};


#[component]
pub fn Profile<G: Html>(ctx: ScopeRef) -> View<G> {

    let handle_click_bible = move |_e: Event| {
        console::log_1(&format!("clicked").as_str().into());
    };
    

    let DarkMode(dark_mode) = ctx.use_context::<DarkMode>();
    let toggle = |_| {
        dark_mode.set(!*dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

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
                div(class="card-wrapper") {
                    div(class="card") {
                        div(style="width:100%; display: flex; justify-content: center;") {
                            img(class="profile-img-big", loading="lazy", 
                                src="/assets/img/avatar_1.webp", 
                                alt="My Profile")
                        }

                        div(class="card__subtext") {
                            span(){"Display Name: "}
                        }
                        div(class="card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }

                    div(class="card") {

                        div(class="card__subtext") {
                            span(){"Real Name: "}
                            span(){"Email: "}
                            span(){"Phone: "}
                            div(class="row__seperator")
                            span(){"Age: "}
                            span(){"Gender: "}
                            span(){"Race: "}
                            span(){"Job: "}
                        }
                        div(class="card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }
                    div(class="card") {

                        div(class="card__subtext") {
                            span(){"Church Name: "}
                            span(){"Cell Group: "}
                            span(){"Other Info: "}
                            div(class="row__seperator")
                            span(){"Address: "}
                            span(){"City: "}
                            span(){"ZipCode: "}
                            span(){"State: "}
                            span(){"Country: "}
                        }
                        div(class="card-buttons") {
                            button(class="cbutton status-button", on:click=handle_click_bible) {"Edit"}
                        }
                    }
                }

            }

            div(class="content-section") {
                div(class="content-section-title") { "SYSTEM PREFERENCES" }
                div(class="cards") {
                    div(class="card") {
                        
                        div(class="card__subtext") {

                            div(class="row nowrap") {
                                div(class="col-4 left nowrap") { "Appearance:" }
                                div(class="col-8 right nowrap") {
                                    label(class="theme-switch", for="chkSwitch") {
                                        input(id="chkSwitch", type="checkbox", checked=*dark_mode.get(), on:click=toggle)
                                        div(class="theme-switch-slider round")
                                    }
                                }
                                    
                               
                            }
                            div(class="row__seperator")
                            div(class="row nowrap") {
                                div(class="col-4 left nowrap"){"Accent Color: "}
                                div(class="col-8 right nowrap") {""}
                            }
                            div(class="row__seperator")
                            div(class="row nowrap") {
                                div(class="col-4 left nowrap"){"Background Image: "}
                                div(class="col-8 right nowrap") {""}
                            }
                            
                        }
                       
                    }
                }
            }
            footer::ContentFooter()
        }
    }
}
