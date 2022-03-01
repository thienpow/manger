use sycamore::prelude::*;
use web_sys::{Event};

use crate::{components::footer, context::{AppState}};


#[component]
pub fn Profile<G: Html>(ctx: ScopeRef) -> View<G> {

    let app_state = ctx.use_context::<AppState>();
    
    let handle_click_bible = move |_: Event| {
        //console::log_1(&format!("clicked").as_str().into());
    };

    view! { ctx,

        div(class="wrapper") {
            div(class="main-container") {

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
                                                input(id="chkSwitch", type="checkbox", 
                                                    checked=*app_state.dark_mode.get(), 
                                                    on:click=move |_| app_state.toggle_dark_mode())
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
        
    }
}
