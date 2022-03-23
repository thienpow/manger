
use sycamore::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, Event, console};
use crate::{
    pages::bible::{
        self, toc::TOC, navbar::NavBar, content::Content, 
        store::{
            BibleState
        }
    }
};


#[component]
pub fn Bible<G: Html>(cx: Scope) -> View<G> {

    bible::store::initialize(cx);

    let bible_state = use_context::<BibleState>(cx);
    
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

        div(class="wrapper",
            on:keydown=on_keydown,
        ) {

            TOC()

            // bible verse content area
            div(class="main-container", 
                tabindex="0", 
                style=format!("{}", if bible_state.verses.get().iter().len() > 0 {""} else {"display: none;"})) {


                div(class="bible-wrapper") {
                        NavBar {}
                        Content {}
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
