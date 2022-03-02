
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::ScopeSpawnFuture;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::api::bible::get_toc;
use crate::context::AppState;
use crate::context::BibleBookItem;
use crate::context::ChapterItem;


fn clear_selected_button(selector: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.query_selector(selector).unwrap();
    if element.is_some() {
        let element = element.unwrap();
        element.set_class_name("");
    }
}

#[component]
pub fn BookItem<G: Html>(ctx: ScopeRef, book: RcSignal<BibleBookItem>) -> View<G> {

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
        clear_selected_button(".toc-menu-selected");
        
        let toc_item_ref = toc_item_ref.get::<DomNode>().inner_element().dyn_into::<Element>().unwrap();
        toc_item_ref.set_class_name("toc-menu-selected");
        
        
        let list = document.get_element_by_id("book_list").unwrap();
        list.set_scroll_top(((id-1) * toc_item_ref.client_height())+15);
        //console::log_1(&format!("book_list.scroll_top() == {}",  i).as_str().into());

        app_state.selected_bible_book.set(BibleBookItem {book_id: id, book_name: book_name, chapters: chapters});
        app_state.reset_chapters(chapters);
        app_state.selected_bible_chapter.set(ChapterItem {id: 0, name: "".to_string()});
        clear_selected_button(".chapter-menu-selected");

        let list = document.get_element_by_id("chapter_list").unwrap();
        list.set_scroll_top(53+15);
    };
 
    let span_style = if app_state.selected_bible_book.get().book_id == id {"toc-menu-selected"} else {""};

    view! { ctx,
        span(ref=toc_item_ref, 
            class=span_style,
            on:click=move |_| handle_toc_click(id, book_name.clone(), chapters)
        ) {
            (book_name_span)
        }
    }
}

#[component]
async fn BookList<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
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
                    BookItem(book)
                },

            key: |book| book.get().book_id,
        }
        div(style="height: 53px")
        
    }
}

#[component]
fn ChapterItem<G: Html>(ctx: ScopeRef, chapter: RcSignal<crate::context::ChapterItem>) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    // Make `todo` live as long as the scope.
    let chapter = ctx.create_ref(chapter);

    let chapter_item_ref = ctx.create_node_ref();


    let handle_chapter_click = |id: i32| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        clear_selected_button(".chapter-menu-selected");

        let chapter_item_ref = chapter_item_ref.get::<DomNode>().inner_element().dyn_into::<Element>().unwrap();
        chapter_item_ref.set_class_name("chapter-menu-selected");
        
        let book_list = document.get_element_by_id("chapter_list").unwrap();
        book_list.set_scroll_top(((id-1) * chapter_item_ref.client_height())+15);

        app_state.selected_bible_chapter.set(ChapterItem {id: id, name: id.to_string()});
    };
 

    let span_style = if app_state.selected_bible_chapter.get().id == chapter.get().id {"chapter-menu-selected"} else {""};

    view! { ctx,
        span(
            ref=chapter_item_ref, 
            class=span_style,
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
    let app_state = ctx.use_context::<AppState>();

    let book_list_ref = ctx.create_node_ref();

    ctx.spawn_future(async move {
        loop {
            TimeoutFuture::new(60).await;

            if app_state.selected_bible_book.get().book_id > 0 {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let list = document.get_element_by_id("book_list").unwrap();
                list.set_scroll_top(((app_state.selected_bible_book.get().book_id-1) * 53)+15);
                app_state.reset_chapters(app_state.selected_bible_book.get().chapters);
            
                
                let list = document.get_element_by_id("chapter_list").unwrap();
                let id = app_state.selected_bible_chapter.get().id;
                if id > 0 {
                    list.set_scroll_top(((id-1) * 53)+15);
                } else {
                    list.set_scroll_top(53+15);
                }
                
                //TODO: reset verses here


                break;
            }
            
        };
        
        

    });


    let show_bar = move |show: bool| {
        let window = web_sys::window().unwrap();
        if window.inner_width().unwrap().as_f64().unwrap() <= 540.0 {
            let document = window.document().unwrap();
            let element = document.query_selector(".toc-bar-left").unwrap().unwrap();

            if app_state.selected_bible_book.get().book_id == 0 || app_state.selected_bible_chapter.get().id == 0 {
                element.set_attribute("style", "position:relative; left: 0px;transition: 0.1s;").unwrap();
            } else {
                if show {
                    element.set_attribute("style", "position:relative; left: 0px;transition: 0.1s;").unwrap();
                } else {
                    element.set_attribute("style", "position:absolute; left: -145px;transition: 0.1s;").unwrap();
                }
            }

        }
    };

    view! { ctx,
        
        div(class="toc-bar-left", 
            style=(if app_state.selected_bible_book.get().book_id == 0 {"left: 0px;"} else {""}),
            on:mouseenter=move |_| show_bar(true),
            on:mouseleave=move |_| show_bar(false)
        ) {

            div(class="toc-title-left") {
                "BOOKS"
            }
            div(class="row")
            div(ref=book_list_ref, id="book_list", class="toc-wrapper") {

                div(class="toc-menu") {
                    Suspense {
                        fallback: view! { ctx, "Loading..." },
                        BookList {}
                    }
                }
                
            }

            div(style="height: 53px;")
            
            div(class="toc-title-left") {
                "CHAPTERS"
            }
            div(class="row")
            div(id="chapter_list", class="toc-wrapper") {

                div(class="toc-menu") {
                               
                    ChapterList {}
                    
                    
                }
            }

        }
        
    }
}
