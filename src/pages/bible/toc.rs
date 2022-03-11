
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use crate::pages::bible::{ 
    self, 
    store::{
        ChapterItem, BibleState, BibleBookItem, 
    }
};

#[component]
pub fn BookItem<G: Html>(ctx: ScopeRef, book: RcSignal<BibleBookItem>) -> View<G> {
    let bible_state = ctx.use_context::<BibleState>();

    let book = book.get();
    let id = book.book_id;
    let book_name: String = book.book_name.clone();
    let chapters = book.chapters;
    let book_name_span = book.book_name.clone();

    let handle_toc_click = |book_id: i32, book_name:  String, chapters: i32| {
        bible_state.selected_bible_book.set(BibleBookItem {book_id, book_name, chapters});
        bible_state.selected_bible_chapter.set(ChapterItem {id: 1, name: "1".to_string()});
        bible::util::reload_chapter_data(ctx);

        bible::util::scroll_to_selected_chapter(ctx, 560);
        bible::util::scroll_to_selected_book(ctx, 60);
    };

    view! { ctx,
        span(
            id=(format!("book-item-{}", id)),
            class=(if bible_state.selected_bible_book.get().book_id == id {
                "toc-menu-selected"
            } else {
                ""
            }),
            on:click=move |_| handle_toc_click(id, book_name.clone(), chapters)
        ) {
            (book_name_span)
        }
    }
}

#[component]
async fn BookList<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let bible_state = ctx.use_context::<BibleState>();
    bible_state.init_bible_books().await;

    let filtered_books = ctx.create_memo(|| {
        bible_state
            .bible_books
            .get()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,
        div(style="height: 53px;")
        Keyed {
            iterable: filtered_books,
            view: |ctx, book | 

                view! { ctx,
                    BookItem(book)
                },

            key: |book| book.get().book_id,
        }
        div(id="book-item-67", style="height: 53px;")
        
    }
}

#[component]
fn ChapterItem<G: Html>(ctx: ScopeRef, chapter: RcSignal<ChapterItem>) -> View<G> {
    let id = chapter.get().id;
    let bible_state = ctx.use_context::<BibleState>();
    
    let handle_chapter_click = |id: i32| {
        bible_state.selected_bible_chapter.set(ChapterItem {id, name: id.to_string()});
        bible::util::reload_chapter_data(ctx);
        bible::util::scroll_to_selected_chapter(ctx, 0);
    };

    view! { ctx,
        span(
            id=(format!("chapter-item-{}", id)),
            class=if bible_state.selected_bible_chapter.get().id == id {"chapter-menu-selected"} else {""},
            on:click=move |_| handle_chapter_click(id)
        ) {
            (id)
        }
    }
}


#[component]
fn ChapterList<G: Html>(ctx: ScopeRef) -> View<G> {
    let bible_state = ctx.use_context::<BibleState>();

    let filtered_chapters = ctx.create_memo(|| {
        bible_state
            .chapters
            .get()
            .iter()
            .filter(|chapter| bible_state.selected_bible_book.get().chapters >= chapter.get().id)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { ctx,
        div(style="height: 53px;white-space: nowrap;")
        Keyed {
            iterable: filtered_chapters,
            view: |ctx, chapter| view! { ctx,
                ChapterItem(chapter)
            },
            key: |chapter| chapter.get().id,
        }
        div(id=format!("chapter-item-{}", bible_state.selected_bible_book.get().chapters+1),
            style="height: 53px;white-space: nowrap;")
    }
}


#[component]
pub fn TOC<G: Html>(ctx: ScopeRef) -> View<G> {

    let bible_state = ctx.use_context::<BibleState>();

    let book_list_ref: &NodeRef<G> = ctx.create_node_ref();
    let chapter_list_ref = ctx.create_node_ref();

    

    if bible_state.chapters.get().len() == 0 {
        bible_state.init_chapters(150);
    }
    
    view! { ctx,
        
        div(id="toc-bar-left", class=(
            if bible_state.selected_bible_book.get().book_id == 0 || bible_state.selected_bible_chapter.get().id == 0 {
                "toc-bar-left"
            } else {
                if *bible_state.show_bible_toc.get() || *bible_state.pin_bible_toc.get() {
                    "toc-bar-left"
                } else {
                    "toc-bar-left toc-bar-left-hide"
                }
            }
        ), 
            on:mouseenter=move |_| bible_state.show_bible_toc.set(true),
            on:mouseleave=move |_| bible_state.show_bible_toc.set(false)
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
            div(ref=chapter_list_ref, id="chapter_list", class="toc-wrapper") {

                div(class="toc-menu") {
                               
                    ChapterList {}
                    
                    
                }
            }

        }
        
    }
}
