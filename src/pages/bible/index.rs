use crate::pages::bible::{
    self, content::Content, intro::Intro, navbar::NavBar, store::BibleState, toc::TOC,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{console, Event, KeyboardEvent};

#[component]
pub fn Bible<G: Html>(cx: Scope) -> View<G> {
    bible::store::initialize(cx);

    let bible_state = use_context::<BibleState>(cx);
    spawn_local_scoped(cx, async move {
        bible_state.load_books().await;
        bible_state.load_verses().await;
    });

    //TODO: key up/down = change chapter
    // key left/right = change page

    /*
    37 = left arrow
    38 = top arrow
    39 = right arrow
    40 = bottom arrow
    32 = spacebar
    8 = backspace
    13 = enter
     */
    let key_code = create_signal(cx, 0);
    let on_keydown = move |e: Event| {
        //e.current_target().unwrap().
        let key = e.unchecked_into::<KeyboardEvent>();

        console::log_1(&format!("{}", key.key()).as_str().into());
        key_code.set(key.key_code());
    };

    view! { cx,
        meta(name="apple-mobile-web-app-status-bar-style", content="var(--content-text-bg)")
        div(class="wrapper",
            on:keydown=on_keydown,
        ) {

            TOC()

            // bible verse content area
            div(class=(
                    if bible_state.selected_bible_book.get().book_id == 0 || bible_state.selected_bible_chapter.get().id == 0 {
                        "main-container show"
                    } else {
                        if *bible_state.show_bible_toc.get() || *bible_state.pin_bible_toc.get() {
                            "main-container show"
                        } else {
                            "main-container hide"
                        }
                    }
                ),
                tabindex="0",
                style=format!(
                    "{}",
                    if bible_state.display_verses.get().iter().len() > 0 {""} else {"display: none;"}
                ),

            ) {


                div(class="bible-wrapper") {
                        NavBar {}
                        Content {}
                }
            }

            Intro {}


        }

    }
}
