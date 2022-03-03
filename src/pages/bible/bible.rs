use sycamore::prelude::*;
use crate::{pages::{bible::toc::TOC}, context::AppState};


#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(style="display: flex;") {
                    TOC()
                    div(class="bible-content-area") {

                        div(class="content-section-title", style="justify-self: center; align-items: center; width: 100%;transition: 0.1s;"){
                            (if app_state.selected_bible_book.get().book_id > 0 {
                                let chapter_text = if app_state.selected_bible_chapter.get().id > 0 {app_state.selected_bible_chapter.get().id.to_string()} else {"".to_string()};
                                format!("{}{}{}", app_state.selected_bible_book.get().book_name, " >> ", chapter_text)
                            } else {"".to_string()})
                        }
                    }
                }

                

                
                
                
            }

        }
         
    }
}
