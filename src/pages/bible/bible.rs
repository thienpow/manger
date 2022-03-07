use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::*, futures::ScopeSpawnFuture};
use web_sys::{Element, console};
use crate::{pages::{bible::{toc::TOC, self}}, context::AppState};
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

            TOC()
            div(class="main-container") {
                div(class="bible-content-header"){
                    
                    i(class=(if *app_state.pin_bible_toc.get() {"gg-chevron-double-right-r"} else {"gg-chevron-double-left-r"}), 
                        style="margin-right:12px;cursor: pointer;",
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

                div(class="bible-content") {
                    div(class="bible-verse-content-wrapper") {

                        //scroll_to_previous_page
                        div(style="White-space: nowrap;height: 100%; width: 48px; border-top-left-radius: 12px;background-color: var(--content-text-bg);",
                            on:click=move |_| bible::util::scroll_to_previous_page(&ctx, 60))

                        div(id="bible-verse-content", 
                            class="bible-verse-content",
                            on:click=move |_| if web_sys::window().unwrap().match_media("(max-width: 420px)").unwrap().unwrap().matches() {
                                app_state.show_bible_toc.set(false);
                                app_state.pin_bible_toc.set(false);
                            }) {
                            Keyed {
                                iterable: get_filtered_verses,
                                view: |ctx, verse| view! { ctx,
                                    VerseItem(verse)
                                },
                                key: |verse| verse.get().verse,
                            }
                        }

                        //scroll_to_next_page
                        div(style="height: 100%; width: 48px; border-top-right-radius: 12px;background-color: var(--content-text-bg);",
                            on:click=move |_| bible::util::scroll_to_next_page(&ctx))
                    }
                }
                
            }

        }
         
    }
}
