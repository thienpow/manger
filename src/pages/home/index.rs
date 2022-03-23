use sycamore::prelude::*;


use crate::{components, pages::home::{sidebar::SidebarLeft, pwa_install::PWAInstall}};


#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {

    view! { cx,

        div(class="wrapper") {

            SidebarLeft()

            div(class="main-container") {

                article(class="content-wrapper") {
                    div(class="content-section") {
                        header(class="content-section-title") {
                            "Welcome to use Manger"
                        }
                        div(class="card-wrapper") {

                            PWAInstall()

                            div(class="card") {
                                span() {"Bible image here"}
                                div(class="card__subtext") {"Please choose a preferred Bible language."}
                                div(class="card-buttons") {
                                    button(class="cbutton status-button") {"中文简体"}
                                    div(class="menu")
                                }
                            }
                        }
                        
                    }
                    
                }
            }

            components::contactbar::ContactBar()
        }

                            
    }
}
