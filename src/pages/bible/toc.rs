
use js_sys::*;
use serde::Deserialize;
use serde::Serialize;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Element;
use web_sys::console;
use crate::api::bible::get_toc;
use crate::context::AppState;
use crate::context::BibleBookItem;



#[derive(Serialize, Deserialize)]
struct SVOption { behavior: String, block: String, inline: String }
    
#[component]
pub fn TOCItem<G: Html>(ctx: ScopeRef, book: RcSignal<BibleBookItem>) -> View<G> {

    let app_state = ctx.use_context::<AppState>();

    let toc_item_ref = ctx.create_node_ref();
    let book = book.get();
    let id = book.book_id;
    let book_name: String = book.book_name.clone();
    let chapters = book.chapters;
    let book_name_span = book.book_name.clone();

    let handle_toc_click = |id: i32, book_name:  String, chapters: i32| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.query_selector(".toc-menu-selected").unwrap();
        if element.is_some() {
            let element = element.unwrap();
            element.set_class_name("");
        }
        
        let toc_item_ref = toc_item_ref.get::<DomNode>().inner_element().dyn_into::<Element>().unwrap();
        toc_item_ref.set_class_name("toc-menu-selected");
        
        
        let book_list = document.get_element_by_id("book_list").unwrap();
        book_list.set_scroll_top(((id-1) * toc_item_ref.client_height())+15);
        //console::log_1(&format!("book_list.scroll_top() == {}",  i).as_str().into());

        app_state.selected_bible_book.set(BibleBookItem {book_id: id, book_name: book_name, chapters: chapters});
        app_state.reset_chapters(chapters);
    };
 
    view! { ctx,
        span(ref=toc_item_ref, on:click=move |_| handle_toc_click(id, book_name.clone(), chapters)) {
            (book_name_span)
        }
    }
}

#[component]
async fn TOCList<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    if app_state.bible_books.get().len() == 0 {
        let toc = get_toc().await.unwrap_or_default();
        app_state.init_bible_books(toc.books);
    }
    

    let filtered_books = ctx.create_memo(|| {
        app_state
            .bible_books
            .get()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,
        div(style="height: 53px")
        Keyed {
            iterable: filtered_books,
            view: |ctx, book | 

                view! { ctx,
                    TOCItem(book)
                },

            key: |book| book.get().book_id,
        }
        div(style="height: 53px")
        
    }
}

#[component]
fn ChapterItem<G: Html>(ctx: ScopeRef, chapter: RcSignal<crate::context::ChapterItem>) -> View<G> {
    let _app_state = ctx.use_context::<AppState>();
    // Make `todo` live as long as the scope.
    let chapter = ctx.create_ref(chapter);

    let chapter_item_ref = ctx.create_node_ref();


    let handle_chapter_click = |id: i32| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.query_selector(".chapter-menu-selected").unwrap();
        if element.is_some() {
            let element = element.unwrap();
            element.set_class_name("");
        }
        
        let chapter_item_ref = chapter_item_ref.get::<DomNode>().inner_element().dyn_into::<Element>().unwrap();
        chapter_item_ref.set_class_name("chapter-menu-selected");
        
        let book_list = document.get_element_by_id("chapter_list").unwrap();
        book_list.set_scroll_top(((id-1) * chapter_item_ref.client_height())+15);
    };
 

    view! { ctx,
        span(
            ref=chapter_item_ref, 
            on:click=move |_| handle_chapter_click(chapter.get().id)
        ) {
            (chapter.get().id)
        }
    }
}


#[component]
fn ChapterList<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    let filtered_chapters = ctx.create_memo(|| {
        app_state
            .chapters
            .get()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,
        div(style="height: 53px")
        Keyed {
            iterable: filtered_chapters,
            view: |ctx, chapter| view! { ctx,
                ChapterItem(chapter)
            },
            key: |chapter| chapter.get().id,
        }
        div(style="height: 53px")
    }
}



#[component]
pub fn TOC<G: Html>(ctx: ScopeRef) -> View<G> {
    

    view! { ctx,
        
        div(class="toc-bar-left") {

            div(class="toc-title-left") {
                "TABLE OF CONTENTS"
            }
            div(id="book_list", class="toc-wrapper") {

                div(class="toc-menu") {
                    Suspense {
                        fallback: view! { ctx, "Loading..." },
                        TOCList {}
                    }
                }
                
            }

            div(style="height: 53px;")
            
            div(class="toc-title-left") {
                "CHAPTERS"
            }
            div(id="chapter_list", class="toc-wrapper") {

                div(class="toc-menu") {
                               
                    ChapterList {}
                    
                    
                }
            }

        }
        
    }
}
