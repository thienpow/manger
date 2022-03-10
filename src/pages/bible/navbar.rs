

use sycamore::prelude::*;

use crate::store::AppState;

#[component]
pub fn TextSizeButton<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    view! { ctx,
        button(class="text-size-button", style="font-size:12pt;", on:click=move |_| app_state.verse_text_size.set(*app_state.verse_text_size.get()-1)) {"A"}
        div(class="icon-gap")
        button(class="text-size-button", style="font-size:18pt;", on:click=move |_| app_state.verse_text_size.set(*app_state.verse_text_size.get()+1)) {"A"}
        div(class="icon-gap")
    }
}

#[component]
pub fn NavBar<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    
    view! { ctx,
        nav(class="navbar"){
                    
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
            div(class="menu-gap nowrap")
            div(class="navbar-menu-right") {
                TextSizeButton()
            }
        }

    }
}