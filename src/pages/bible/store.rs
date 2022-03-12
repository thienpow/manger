use serde::{Serialize, Deserialize};
use sycamore::prelude::{RcSignal, create_rc_signal, ScopeRef};
use crate::pages::bible::api;

pub fn initialize(ctx: ScopeRef) {

    let window = web_sys::window().unwrap();
    let _document = window.document().unwrap();
    

    match window.local_storage().unwrap() {
        Some(_local_storage) => {

            let bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>> = create_rc_signal(Vec::new());
            let chapters: RcSignal<Vec<RcSignal<ChapterItem>>> = create_rc_signal(Vec::new());
            let verses: RcSignal<Vec<RcSignal<VerseItem>>> = create_rc_signal(Vec::new());
            let loaded_verses: RcSignal<Vec<VerseItem>> = create_rc_signal(Vec::new());
            let loaded_book: RcSignal<i32> = create_rc_signal(0);
            
            let selected_bible_book: RcSignal<BibleBookItem> = create_rc_signal(BibleBookItem {book_id: 0, book_name: "".to_string(), chapters: 0 });
            let selected_bible_chapter: RcSignal<ChapterItem> = create_rc_signal(ChapterItem {id: 0, name: "".to_string() });
            let show_bible_toc: RcSignal<bool> = create_rc_signal(false);
            let pin_bible_toc: RcSignal<bool> = create_rc_signal(true);
            let current_verse_page: RcSignal<i32> = create_rc_signal(0);
            let current_verse_scroll_x: RcSignal<f64> = create_rc_signal(0.0);
            let verse_text_size: RcSignal<i32> = create_rc_signal(12);
                    
            let selection_first_verse: RcSignal<VerseItem> = create_rc_signal(VerseItem{
                book_id: 0,
                chapter: 0,
                verse: 0,
                text: "".to_string()
            });

            let bible_state = BibleState {
                bible_books,
                chapters,
                verses,
                loaded_verses,
                loaded_book,
                selected_bible_book,
                selected_bible_chapter,
                show_bible_toc,
                pin_bible_toc,
                current_verse_page,
                current_verse_scroll_x,
                verse_text_size,
                selection_first_verse
            };
            ctx.provide_context(bible_state);
        
        },
        _ => {}
    }
}

pub struct BibleState {
    pub bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>>,
    pub chapters: RcSignal<Vec<RcSignal<ChapterItem>>>,
    pub verses: RcSignal<Vec<RcSignal<VerseItem>>>,
    pub loaded_verses: RcSignal<Vec<VerseItem>>,
    pub loaded_book: RcSignal<i32>,
    pub selected_bible_book: RcSignal<BibleBookItem>,
    pub selected_bible_chapter: RcSignal<ChapterItem>,
    pub show_bible_toc: RcSignal<bool>,
    pub pin_bible_toc: RcSignal<bool>,
    pub current_verse_page: RcSignal<i32>,
    pub current_verse_scroll_x: RcSignal<f64>,
    pub verse_text_size: RcSignal<i32>,
    pub selection_first_verse: RcSignal<VerseItem>,
}

impl BibleState  {

    pub async fn load_chapter_data(&self) {

        let book_id = self.selected_bible_book.get().book_id;
        if *self.loaded_book.get() != book_id {

            //TODO: check if indexedDB got downloaded the verses or not. if got then dont need to call api to download again.
            let loaded_verses = api::get_book_data("en/kjv".to_string(), book_id).await.unwrap().verses;
            self.loaded_book.set(book_id);
            self.loaded_verses.set(loaded_verses);
        }

        self.reset_verses();
    }

    pub async fn init_bible_books(&self) {

        if self.bible_books.get().len() == 0 {
            let toc = api::get_toc().await.unwrap_or_default();

            self.bible_books.set(Vec::new());
            for b in toc.books.iter() {
                self.bible_books.set(
                    self.bible_books
                        .get()
                        .as_ref()
                        .clone()
                        .into_iter()
                        .chain(Some(create_rc_signal(BibleBookItem {
                            book_id: b.book_id,
                            book_name: b.book_name.clone(),
                            chapters: b.chapters
                        })))
                        .collect()
                )
            }
        }
    }

    
    pub fn init_chapters(&self, max: i32) {
        self.chapters.set(Vec::new());
        for n in 1..max+1 {
            self.chapters.set(
                self.chapters
                    .get()
                    .as_ref()
                    .clone()
                    .into_iter()
                    .chain(Some(create_rc_signal(ChapterItem {
                        id: n,
                        name: n.to_string(),
                    })))
                    .collect()
            )
        };
    }


    fn reset_verses(&self) {
        let chapter_id = self.selected_bible_chapter.get().id;
        let loaded_verses = self.loaded_verses.get().iter().cloned().filter(|v| v.chapter == chapter_id).collect::<Vec<VerseItem>>();

        self.verses.set(Vec::new());
        for v in loaded_verses.iter() {
            //console::log_1(&"iter".into());
            self.verses.set(
                self.verses
                    .get()
                    .as_ref()
                    .clone()
                    .into_iter()
                    .chain(Some(create_rc_signal(VerseItem {
                        book_id: v.book_id,
                        chapter: v.chapter,
                        verse: v.verse,
                        text: v.text.clone()
                    })))
                    .collect()
            )
        }
        
    }
}


#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BibleBookItem {
    pub book_id: i32,
    pub book_name: String,
    pub chapters: i32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct ChapterItem {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataItem {
    pub name: String,
    pub short_code: String,
    pub language: String,
    pub book_id: i32
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct VerseItem {
    pub book_id: i32,
    pub chapter: i32,
    pub verse: i32,
    pub text: String
}
