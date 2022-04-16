

use sycamore::{prelude::*};
use sycamore::suspense::Suspense;
use crate::{pages::bible::{ 
    self, 
    store::{
        ChapterItem, BibleState, BibleBookItem, 
    }
}, components::toast::{ToastProps, self}};

#[component]
pub fn BookItem<G: Html>(cx: Scope, book: RcSignal<BibleBookItem>) -> View<G> {
    let bible_state = use_context::<BibleState>(cx);

    let book = book.get();
    let id = book.book_id;
    let book_name: String = book.book_name.clone();
    let chapters = book.chapters;
    let book_name_span = book.book_name.clone();

    let handle_toc_click = |cx: Scope, book_id: u32, book_name:  String, chapters: u32| {
        let book_name_clone = book_name.clone();
        bible_state.selected_bible_book.set(BibleBookItem {book_id, book_name, chapters});
        bible_state.selected_bible_chapter.set(ChapterItem {id: 1, name: "1".to_string()});
        bible::util::reload_chapter_data(cx);

        bible::util::scroll_to_selected_book(cx, 60);
        bible::util::scroll_to_selected_chapter(cx, 560);
        bible::util::scroll_to_first_page(cx, 1000);

        toast::show(cx, ToastProps{title: "title here".to_string(), text: book_name_clone, icon_url: "".to_string()});
    };

    view! { cx,
        span(
            id=(format!("book-item-{}", id)),
            class=(if bible_state.selected_bible_book.get().book_id == id {
                "toc-menu-selected"
            } else {""}),
            on:click=move |_| handle_toc_click(cx, id, book_name.clone(), chapters)
        ) {
            (book_name_span)
        }
    }
}

#[component]
async fn BookList<G: Html>(cx: Scope<'_>) -> View<G> {
    let bible_state = use_context::<BibleState>(cx);
    
    let filtered_books = create_memo(cx, || {
        bible_state
            .bible_books
            .get()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { cx,
        div(style="height: 53px;")
        Keyed {
            iterable: filtered_books,
            view: |cx, book | 

                view! { cx,
                    BookItem(book)
                },

            key: |book| book.get().book_id,
        }
        div(id="book-item-67", style="height: 53px;")
        
    }
}

#[component]
fn ChapterItem<G: Html>(cx: Scope, chapter: RcSignal<ChapterItem>) -> View<G> {
    let id = chapter.get().id;
    let bible_state = use_context::<BibleState>(cx);
    
    let handle_chapter_click = |cx: Scope, id: u32| {
        bible_state.selected_bible_chapter.set(ChapterItem {id, name: id.to_string()});
        bible::util::reload_chapter_data(cx);
        bible::util::scroll_to_selected_chapter(cx, 60);
        bible::util::scroll_to_first_page(cx, 560);
    };

    view! { cx,
        span(
            id=(format!("chapter-item-{}", id)),
            class=(if bible_state.selected_bible_chapter.get().id == id {
                "toc-menu-selected"
            } else {""}),
            on:click=move |_| handle_chapter_click(cx, id)
        ) {
            (id)
        }
    }
}


#[component]
fn ChapterList<G: Html>(cx: Scope) -> View<G> {
    let bible_state = use_context::<BibleState>(cx);

    let filtered_chapters = create_memo(cx, || {
        bible_state
            .chapters
            .get()
            .iter()
            .filter(|chapter| bible_state.selected_bible_book.get().chapters >= chapter.get().id)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { cx,
        div(style="height: 53px;white-space: nowrap;")
        Keyed {
            iterable: filtered_chapters,
            view: |cx, chapter| view! { cx,
                ChapterItem(chapter)
            },
            key: |chapter| chapter.get().id,
        }
        div(id=format!("chapter-item-{}", bible_state.selected_bible_book.get().chapters+1),
            style="height: 53px;white-space: nowrap;")
    }
}


#[component]
pub fn TOC<G: Html>(cx: Scope) -> View<G> {

    let bible_state = use_context::<BibleState>(cx);

    if bible_state.chapters.get().len() == 0 {
        bible_state.load_chapters(150);
    }


    view! { cx,
        
        div(id="toc-bar-left", class=(
            if bible_state.selected_bible_book.get().book_id == 0 || bible_state.selected_bible_chapter.get().id == 0 {
                "toc-bar-left show"
            } else {
                if *bible_state.show_bible_toc.get() || *bible_state.pin_bible_toc.get() {
                    "toc-bar-left"
                } else {
                    "toc-bar-left hide"
                }
            }
        )) {

            div(class="toc-title-left") {
                "BOOKS"
            }
            div(class="row")
            div(id="book_list", class="toc-wrapper") {

                div(class="toc-menu") {
                    Suspense {
                        fallback: view! { cx, "Loading..." },
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
