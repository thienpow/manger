use sycamore::prelude::*;
use crate::{pages::{bible::toc::TOC, app}, context::AppState};


#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    
    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(class="row nowrap") {
                    div(class="col-2 left nowrap"){
                        TOC()
                    }
                    div(class="col-10 left nowrap") {
                        div(class="bible-content-area") {
                            "aahelo" (app_state.selected_bible_book.get().book_name)
                            
                        }
                    }
                }

                

                
                
                
            }

        }
         
    }
}
