
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::ScopeSpawnFuture;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use crate::api::bible::get_toc;
use crate::context::AppState;
use crate::context::BibleBookItem;
use crate::context::ChapterItem;


fn _clear_selected_button(selector: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    match document.query_selector(selector).unwrap() {
        Some(element) => {
            element.set_class_name("");
        },
        _ => ()
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
        
        app_state.selected_bible_book.set(BibleBookItem {book_id: id, book_name: book_name, chapters: chapters});
        match document.get_element_by_id("book_list") {
            Some(list) => {
                list.set_scroll_top(((id-1) * 53)+15);
            },
            _ => {}
        }
        
        app_state.selected_bible_chapter.set(ChapterItem {id: 0, name: "".to_string()});
        match document.get_element_by_id("chapter_list") {
            Some(list) => {
                list.set_scroll_top(53+15);
            },
            _ => {}
        }
        
        
    };

    view! { ctx,
        span(ref=toc_item_ref, 
            class=if app_state.selected_bible_book.get().book_id == id {"toc-menu-selected"} else {""},
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
fn ChapterItem<G: Html>(ctx: ScopeRef, chapter: &RcSignal<ChapterItem>) -> View<G> {
    let id = chapter.get().id;
    let app_state = ctx.use_context::<AppState>();
    
    let handle_chapter_click = |id: i32| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        let book_list = document.get_element_by_id("chapter_list").unwrap();
        book_list.set_scroll_top(((id-1) * 53)+15);

        app_state.selected_bible_chapter.set(ChapterItem {id: id, name: id.to_string()});
    };

    view! { ctx,
        span(
            class=if app_state.selected_bible_chapter.get().id == id {"chapter-menu-selected"} else {""},
            on:click=move |_| handle_chapter_click(id)
        ) {
            (id)
        }
    }
}


#[component]
fn ChapterList<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();

    if app_state.chapters.get().len() == 0 {
        app_state.init_chapters(150);
    }
    
    let filtered_chapters = ctx.create_memo(|| {
        app_state
            .chapters
            .get()
            .iter()
            .filter(|chapter| app_state.selected_bible_book.get().chapters >= chapter.get().id)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,
        div(style="height: 53px")
        Keyed {
            iterable: filtered_chapters,
            view: |ctx, chapter| view! { ctx,
                ChapterItem(&chapter)
            },
            key: |chapter| chapter.get().id,
        }
        div(style="height: 53px")
    }
}



#[component]
pub fn TOC<G: Html>(ctx: ScopeRef) -> View<G> {
    let app_state = ctx.use_context::<AppState>();
    let mouse_entered = ctx.create_signal(false);

    let book_list_ref = ctx.create_node_ref();

    ctx.spawn_future(async move {
        loop {
            //console::log_1(&format!("TimeoutFuture").as_str().into());
            TimeoutFuture::new(60).await;

            if app_state.selected_bible_book.get().book_id > 0 {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                
                match document.get_element_by_id("book_list") {
                    Some(list) => {
                        list.set_scroll_top(((app_state.selected_bible_book.get().book_id-1) * 53)+15);
                    },
                    _ => break
                }

                match document.get_element_by_id("chapter_list") {
                    Some(list) => {
                        let id = app_state.selected_bible_chapter.get().id;
                        if id > 0 {
                            list.set_scroll_top(((id-1) * 53)+15);
                        } else {
                            list.set_scroll_top(53+15);
                        }
                    },
                    _ => ()
                }
                
                
                //TODO: reset verses here


                break;
            }
            
            
        };
        
        

    });


    
    let get_class = move || {
        let window = web_sys::window().unwrap();
        if window.inner_width().unwrap().as_f64().unwrap() <= 540.0 {
            if app_state.selected_bible_book.get().book_id == 0 || app_state.selected_bible_chapter.get().id == 0 {
                "toc-bar-left"
            } else {
                if *mouse_entered.get() {
                    "toc-bar-left"
                } else {
                    "toc-bar-left toc-bar-left-hide"
                }
            }
        } else {
            "toc-bar-left"
        }
    };


    view! { ctx,
        
        div(id="toc-bar-left", class=get_class(), 
            on:mouseenter=move |_| mouse_entered.set(true),
            on:mouseleave=move |_| mouse_entered.set(false)
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
