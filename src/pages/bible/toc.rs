

use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use crate::api::bible::get_toc;
use crate::api::bible::Book;
use crate::context::AppState;
use crate::context::BibleBook;
use crate::context::SelectedBibleBook;




#[component]
pub fn TOCItem<G: Html>(ctx: ScopeRef, book: Book) -> View<G> {

    let app_state = ctx.use_context::<AppState>();

    let SelectedBibleBook(selected_bible_book) = ctx.use_context::<SelectedBibleBook>();

    let toc_item_ref = ctx.create_node_ref();
    let id = book.book_id;
    let book_name: String = book.book_name.clone();
    let chapters = book.chapters;
    let book_name_span = book.book_name.clone();

    let handle_toc_click = |id: i32, name:  String, chapters: i32| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.query_selector(".toc-menu-selected").unwrap();
        if element.is_some() {
            let element = element.unwrap();
            element.set_class_name("");
        }
        
        toc_item_ref.get::<DomNode>().add_class("toc-menu-selected");

        let selected_bible = BibleBook {id: id, name: name, chapters: chapters};
        selected_bible_book.set(selected_bible);

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

    let toc = get_toc().await.unwrap_or_default();
    let books = ctx.create_signal(toc.books);

    view! { ctx,

        Keyed {
            iterable: books,
            view: |ctx, book | 

                view! { ctx,
                    TOCItem(book)
                },

            key: |book| book.book_id,
        }
        
    }
}

#[component]
fn ChapterItem<G: Html>(ctx: ScopeRef, chapter: RcSignal<crate::context::ChapterItem>) -> View<G> {
    let _app_state = ctx.use_context::<AppState>();
    // Make `todo` live as long as the scope.
    let chapter = ctx.create_ref(chapter);

    let chapter_item_ref = ctx.create_node_ref();

    view! { ctx,
        span(
            ref=chapter_item_ref, 
            //on:click=move |_| handle_chapter_click(chapter)
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
        Keyed {
            iterable: filtered_chapters,
            view: |ctx, chapter| view! { ctx,
                ChapterItem(chapter)
            },
            key: |chapter| chapter.get().id,
        }
    }
}



#[component]
pub fn TOC<G: Html>(ctx: ScopeRef) -> View<G> {

    view! { ctx,
        
        div(class="toc-bar-left") {

            div(class="toc-title-left") {
                "TABLE OF CONTENTS"
            }
            div(class="toc-wrapper") {

                div(class="toc-menu") {
                    Suspense {
                        fallback: view! { ctx, "Loading..." },
                        TOCList {}
                    }
                }
                
            }

            div(class="row-gap")
            
            div(class="toc-title-left") {
                "CHAPTERS"
            }
            div(class="toc-wrapper") {

                div(class="toc-menu") {
                               
                    ChapterList {}
                    
                    
                }
            }

        }
        
    }
}
