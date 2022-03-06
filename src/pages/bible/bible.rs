use sycamore::prelude::*;
use crate::{pages::{bible::toc::TOC}, context::AppState};
use crate::context;

#[component]
fn VerseItem<G: Html>(ctx: ScopeRef, verse: RcSignal<context::VerseItem>) -> View<G> {
    let verse =  verse.get();
    let prefix = format!("[{}:{}]  ", verse.chapter, verse.verse);

    view! { ctx,
        span() {(prefix)}
        span(style=""
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
                    div(class="bible-content") {

                        div(class="content-section-title", 
                            style="display:flex; padding-left: 25px; justify-self: center; align-items: center; width: 100%;transition: 0.3s;height: 24px;",
                            
                        ){
                            i(class=(if *app_state.pin_bible_toc.get() {"gg-chevron-double-right-r"} else {"gg-chevron-double-left-r"}), 
                                style="margin-right:25px;cursor: pointer;",
                                on:click=move |_| {
                                    app_state.pin_bible_toc.set(!*app_state.pin_bible_toc.get())
                                })

                            (if app_state.selected_bible_book.get().book_id > 0 {
                                format!("{}",app_state.selected_bible_book.get().book_name)
                            } else {"".to_string()})

                            i(class=("gg-chevron-double-right"), style="margin-left:4px;margin-right:4px;")

                            (if app_state.selected_bible_book.get().book_id > 0 {
                                format!("{}",app_state.selected_bible_chapter.get().id.to_string())
                            } else {"".to_string()})
                        }
                        div(class="bible-verse-content-wrapper") {
                            div(class="bible-verse-content",
                                style=(if web_sys::window().unwrap().match_media("(max-width: 540px)").unwrap().unwrap().matches() {""} else {"max-width: 580px;"}),
                                on:click=move |_| if web_sys::window().unwrap().match_media("(max-width: 420px)").unwrap().unwrap().matches() {
                                    app_state.show_bible_toc.set(false);
                                    app_state.pin_bible_toc.set(false);
                                },
                        ) {
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
