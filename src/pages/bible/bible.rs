use sycamore::prelude::*;
use crate::{pages::{bible::toc::TOC}, context::AppState};


#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    let handle_bar_hide = move |_| {
        
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.query_selector(".toc-bar-left").unwrap().unwrap();

        if app_state.selected_bible_book.get().book_id == 0 || app_state.selected_bible_chapter.get().id == 0 {
            element.set_attribute("style", "position:relative; left: 0px;transition: 0.1s;").unwrap();
        } else {
            if window.inner_width().unwrap().as_f64().unwrap() <= 540.0 {
                element.set_attribute("style", "position:absolute; left: -145px;transition: 0.1s;").unwrap();
            } else {
                element.set_attribute("style", "position:relative; left: 0px;transition: 0.1s;").unwrap();
            }
        }
    };
    
    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(style="display: flex;") {
                    TOC()
                    div(class="bible-content-area", on:click=handle_bar_hide) {

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
