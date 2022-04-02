use sycamore::{prelude::*, futures::spawn_local_scoped};
use web_sys::Event;

use crate::pages::bible::{db, store::BibleState};

#[component]
pub fn Intro<G: Html>(cx: Scope) -> View<G> {

    let bible_state = use_context::<BibleState>(cx);
    let on_click = move |_: Event| {
        spawn_local_scoped(cx, async move {
            bible_state.delete_bible().await;
            bible_state.load_bible().await;
        });
    };
    
    view! { cx,
        
        blockquote {
            p {"notify/alert status here... e.g 'you haven't pick a book/chapter. or you are disconnected from internet. etc' "}
            button(on:click=on_click) {"reset"}
        }
        
        blockquote {
            p {"Intro/Shortcuts here:"}
            ul {
                li {"intro keyboard shortcuts usage"}
                li {"click to last read verse, show a few short verses in between the last read position"}
                li {"bookmarks, most recent 5 book marks, click more to view full page bookmark list"}
                li {"last edited note/comment"}
                li {"friend's note/comment sharing"}
                li {"switch v-scroll/h-scroll mode"}
                li {"float toolbar in mobile screen, with tool buttons like change font size, switch scroll mode, search, chapter nav, note taking etc"}
            }
        }
        
    }
}
