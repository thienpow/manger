use sycamore::prelude::*;
use crate::store::AppState;

#[component]
pub fn Preference<G: Html>(cx: Scope) -> View<G> {

    let app_state = use_context::<AppState>(cx);
    
    let get_bg_button_style = move |bg: &str| {
        if bg == &*app_state.background.get() {
            "bg-button-selected"
        } else {
            "bg-button"
        }
    };
    view! { cx,
        
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
                            div(class="col-4 left nowrap"){"Background: "}
                            div(class="col-8 right nowrap") {
                                button(class=get_bg_button_style("/assets/img/bg-cloud.webp"),
                                    on:click=move |_| app_state.switch_background("/assets/img/bg-cloud.webp")
                                ){
                                    img(src="/assets/img/bg-cloud.webp", loading="lazy", alt="Cloud Background")
                                }
                                
                                button(class=get_bg_button_style("/assets/img/bg-cross.webp"),
                                    on:click=move |_| app_state.switch_background("/assets/img/bg-cross.webp")
                                ){
                                    img(src="/assets/img/bg-cross.webp", loading="lazy", alt="Cross Background")
                                }
                                
                            }
                        }
                        
                    }
                   
                }
            }
        }

    }
}
