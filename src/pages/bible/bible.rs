use sycamore::prelude::*;
use crate::pages::bible::toc::TOC;


#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(class="row nowrap") {
                    div(class="col-2 left nowrap"){
                        TOC()
                    }
                    div(class="col-10 left nowrap") {
                        div(class="bible-content-area") {
                            "aahelo"
                            
                        }
                    }
                }

                

                
                
                
            }

        }
         
    }
}
