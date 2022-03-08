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
        span(style="font-size:inherit;") {(prefix)}
        span(style="font-size:inherit;"
        ) {
            (verse.text)
            br()br()
        }
    }
}

#[component]
pub fn BackButton<G: Html>(ctx: ScopeRef) -> View<G> {
    let show_button = ctx.create_signal(false);

    view! { ctx,
        div(class="verse-content-nav-panel", style="border-top-left-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_first_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_previous_page(&ctx, 60)
            ) {
                i(class="gg-chevron-left")
            }
        }
    }
}

#[component]
pub fn NextButton<G: Html>(ctx: ScopeRef) -> View<G> {
    let show_button = ctx.create_signal(false);

    view! { ctx,
        div(class="verse-content-nav-panel", style="Wborder-top-right-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_last_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_next_page(&ctx)
            ) {
                i(class="gg-chevron-right")
            }
        }
    }
}

#[component]
pub fn TextSizeButton<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    view! { ctx,
        div(class="menu-gap nowrap", style="width: 48px;")
        button(class="text-size-button", style="font-size:12pt;", on:click=move |_| app_state.verse_text_size.set(*app_state.verse_text_size.get()-1)) {"A"}
        div(class="icon-gap")
        button(class="text-size-button", style="font-size:18pt;", on:click=move |_| app_state.verse_text_size.set(*app_state.verse_text_size.get()+1)) {"A"}
        div(class="icon-gap")
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
                div(class="navbar"){
                    
                    div(class="navbar-menu", style="padding-left: 25px; color:var(--button-inactive);") {
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
                    
                    div(class="navbar-menu-right") {
                        TextSizeButton()
                    }
                }

                div(class="bible-content") {
                    div(class="bible-verse-content-wrapper") {

                        //scroll_to_previous_page
                        BackButton()

                        div(id="bible-verse-content", 
                            class="bible-verse-content",
                            style=format!("font-size:{}pt;", *app_state.verse_text_size.get()),
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
                        NextButton()
                    }
                }
                
            }

        }
         
    }
}
