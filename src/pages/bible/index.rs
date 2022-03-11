
use sycamore::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, Event, console};
use crate::{
    store::AppState,
    pages::bible::{
        self, toc::TOC, navbar::NavBar, content::Content, 
        store::{
            BibleState
        }
    }
};


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
        div(class="verse-content-nav-panel", style="border-top-right-radius: 12px;",
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
pub fn Bible<G: Html>(ctx: ScopeRef) -> View<G> {

    let app_state = ctx.use_context::<AppState>();
    
    bible::store::initialize(ctx);

    let bible_state = ctx.use_context::<BibleState>();
    
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
    let key_code = ctx.create_signal(0);
    let on_keydown = move |e: Event| {
        //e.current_target().unwrap().
        let key = e.unchecked_into::<KeyboardEvent>();

        console::log_1(&format!("{}", key.key()).as_str().into());
        key_code.set(key.key_code());
    };


    let _bible_content_style_height = ctx.create_memo(|| {
        let inner_width: f64 = *app_state.inner_width.get();
        let inner_height: f64 = *app_state.inner_height.get();
        let mut height =  inner_height-58.0;
        if inner_width <= 738.0 {
            height =  inner_height-11.0;
        } 
        format!("height: {}px;", height)
    });

    view! { ctx,

        div(class="wrapper",
            on:keydown=on_keydown,
        ) {

            TOC()

            // bible verse content area
            div(class="main-container", 
                tabindex="0", 
                style=format!("{}", if bible_state.verses.get().iter().len() > 0 {""} else {"display: none;"})) {
                NavBar()

                article(class="bible-content") {
                    div(class="bible-verse-content-wrapper") {

                        //scroll_to_previous_page
                        BackButton()
                        //(*key_code.get())
                        Content()

                        //scroll_to_next_page
                        NextButton()
                    }
                }
            }
            div(class="main-container", tabindex="0", style=format!("{}", if bible_state.verses.get().iter().len() > 0 {"display: none"} else {""})) {

                blockquote {
                    p {"notify/alert status here... e.g 'you haven't pick a book/chapter. or you are disconnected from internet. etc' "}
                }
                
                blockquote {
                    p {"Intro/Shortcuts here:"}
                    ul {
                        li {"intro keyboard shortcuts usage"}
                        li {"click to last read verse, show a few short verses in between the last read position"}
                        li {"bookmarks, most recent 5 book marks, click more to view full page bookmark list"}
                        li {"last edited note/comment"}
                        li {"friend's note/comment sharing"}
                    }
                }
                

                
            }

        }
         
    }
}
