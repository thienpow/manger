use sycamore::prelude::*;
use crate::{pages::{bible::toc::TOC}, context::AppState};
use crate::context;

#[component]
fn VerseItem<G: Html>(ctx: ScopeRef, verse: RcSignal<context::VerseItem>) -> View<G> {
    let verse =  verse.get();
    let prefix = format!("[{}:{}]  ", verse.chapter, verse.verse);

    view! { ctx,
        span() {(prefix)}
        span(
        ) {
            (verse.text)
            br()br()
        }
    }
}

#[component]
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    let get_filtered_verses = ctx.create_memo(|| {
        app_state
            .verses
            .get()
            .iter()
            //.filter(|v| app_state.selected_bible_chapter.get().id == v.get().chapter)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,

        div(class="wrapper") {

            div(class="main-container") {

                div(style="display: flex;") {
                    TOC()
                    div(class="bible-content-area") {

                        div(class="content-section-title", 
                            style="justify-self: center; align-items: center; width: 100%;transition: 0.1s;",
                            on:click=move |_| app_state.pin_bible_toc.set(!*app_state.pin_bible_toc.get()),
                        ){
                            (if app_state.selected_bible_book.get().book_id > 0 {
                                let chapter_text = if app_state.selected_bible_chapter.get().id > 0 {app_state.selected_bible_chapter.get().id.to_string()} else {"".to_string()};
                                format!("{}{}{}{}", if *app_state.pin_bible_toc.get() {">> "} else {"<< "},app_state.selected_bible_book.get().book_name, " >> ", chapter_text)
                            } else {"".to_string()})
                        }
                        div(class="bible-verse-content-wrapper") {
                            div(class="bible-verse-content") {
                                Keyed {
                                    iterable: get_filtered_verses,
                                    view: |ctx, verse| view! { ctx,
                                        VerseItem(verse)
                                    },
                                    key: |verse| verse.get().verse,
                                }
                            }
                        }
                        
                        
                    }
                }

                

                
                
                
            }

        }
         
    }
}
