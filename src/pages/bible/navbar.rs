

use sycamore::prelude::*;

use crate::pages::bible::store::BibleState;

#[component]
pub fn TextSizeButton<G: Html>(ctx: ScopeRef) -> View<G> {
    let bible_state = ctx.use_context::<BibleState>();

    view! { ctx,
        button(class="text-size-button", style="font-size:12pt;", on:click=move |_| bible_state.verse_text_size.set(*bible_state.verse_text_size.get()-1)) {"A"}
        div(class="icon-gap")
        button(class="text-size-button", style="font-size:18pt;", on:click=move |_| bible_state.verse_text_size.set(*bible_state.verse_text_size.get()+1)) {"A"}
        div(class="icon-gap")
    }
}

#[component]
pub fn NavBar<G: Html>(ctx: ScopeRef) -> View<G> {
    let bible_state = ctx.use_context::<BibleState>();
    
    view! { ctx,
        nav(class="navbar"){
                    
            div(class="navbar-menu", style="padding-left: 25px; color:var(--button-inactive);",
                on:click=move |_| {
                    bible_state.pin_bible_toc.set(!*bible_state.pin_bible_toc.get())
                }
            ) {
                i(class=(if *bible_state.pin_bible_toc.get() {"gg-chevron-double-right-r"} else {"gg-chevron-double-left-r"}), 
                style="margin-right:12px;cursor: pointer;")

                (if bible_state.selected_bible_book.get().book_id > 0 {
                    format!("{}",bible_state.selected_bible_book.get().book_name)
                } else {"".to_string()})

                i(class=("gg-chevron-double-right"), style="margin-left:4px;margin-right:4px;")

                (if bible_state.selected_bible_book.get().book_id > 0 {
                    format!("{}",bible_state.selected_bible_chapter.get().id.to_string())
                } else {"".to_string()})

            }
            div(class="menu-gap nowrap")
            div(class="navbar-menu-right") {
                TextSizeButton()
            }
        }

    }
}