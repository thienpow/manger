use crate::pages::bible::api;
use serde::{Deserialize, Serialize};
use sycamore::prelude::{create_rc_signal, provide_context, RcSignal, Scope};

use super::db;

pub fn initialize(cx: Scope) {
    let window = web_sys::window().unwrap();
    let _document = window.document().unwrap();

    match window.local_storage().unwrap() {
        Some(_local_storage) => {
            let bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>> = create_rc_signal(Vec::new());
            let chapters: RcSignal<Vec<RcSignal<ChapterItem>>> = create_rc_signal(Vec::new());
            let display_verses: RcSignal<Vec<RcSignal<VerseItem>>> = create_rc_signal(Vec::new());
            let book_verses: RcSignal<Vec<VerseItem>> = create_rc_signal(Vec::new());
            let full_verses: RcSignal<Vec<VerseItem>> = create_rc_signal(Vec::new());
            let loaded_book: RcSignal<u32> = create_rc_signal(0);
            let full_verses_loading: RcSignal<bool> = create_rc_signal(false);

            let selected_bible: RcSignal<String> = create_rc_signal("en/kjv".to_string());
            let selected_bible_book: RcSignal<BibleBookItem> = create_rc_signal(BibleBookItem {
                book_id: 0,
                book_name: "".to_string(),
                chapters: 0,
            });
            let selected_bible_chapter: RcSignal<ChapterItem> = create_rc_signal(ChapterItem {
                id: 0,
                name: "".to_string(),
            });
            let show_bible_toc: RcSignal<bool> = create_rc_signal(false);
            let pin_bible_toc: RcSignal<bool> = create_rc_signal(true);
            let toc_animating: RcSignal<bool> = create_rc_signal(false);

            let current_verse_page: RcSignal<u32> = create_rc_signal(0);
            let current_verse_scroll_x: RcSignal<f64> = create_rc_signal(0.0);
            let verse_text_size: RcSignal<u32> = create_rc_signal(12);
            let is_bookview: RcSignal<bool> = create_rc_signal(true);

            let selection_first_verse: RcSignal<VerseItem> = create_rc_signal(VerseItem {
                //id: 0,
                book_id: 0,
                chapter: 0,
                verse: 0,
                text: "".to_string(),
            });

            let bible_state = BibleState {
                bible_books,
                chapters,
                display_verses,
                book_verses,
                full_verses,
                loaded_book,
                full_verses_loading,
                selected_bible,
                selected_bible_book,
                selected_bible_chapter,
                show_bible_toc,
                pin_bible_toc,
                toc_animating,
                current_verse_page,
                current_verse_scroll_x,
                verse_text_size,
                is_bookview,
                selection_first_verse,
            };
            provide_context(cx, bible_state);
        }
        _ => {}
    }
}

pub struct BibleState {
    pub bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>>,
    pub chapters: RcSignal<Vec<RcSignal<ChapterItem>>>,
    pub display_verses: RcSignal<Vec<RcSignal<VerseItem>>>,
    pub book_verses: RcSignal<Vec<VerseItem>>,
    pub full_verses: RcSignal<Vec<VerseItem>>,
    pub loaded_book: RcSignal<u32>,
    pub full_verses_loading: RcSignal<bool>,
    pub selected_bible: RcSignal<String>,
    pub selected_bible_book: RcSignal<BibleBookItem>,
    pub selected_bible_chapter: RcSignal<ChapterItem>,
    pub show_bible_toc: RcSignal<bool>,
    pub pin_bible_toc: RcSignal<bool>,
    pub toc_animating: RcSignal<bool>,
    pub current_verse_page: RcSignal<u32>,
    pub current_verse_scroll_x: RcSignal<f64>,
    pub verse_text_size: RcSignal<u32>,
    pub is_bookview: RcSignal<bool>,
    pub selection_first_verse: RcSignal<VerseItem>,
}

impl BibleState {
    pub async fn change_bible(&self, bible: String) {
        self.full_verses_loading.set(true);
        self.selected_bible.set(bible);
        self.bible_books.set(Vec::new());
        self.full_verses.set(Vec::new());
        self.load_books().await;
        self.load_verses().await;
        self.full_verses_loading.set(false);
    }

    pub async fn load_verses(&self) {
        let bible = self.selected_bible.get().to_string();

        //check if indexedDB got downloaded the verses or not. if got then dont need to call api to download again.
        let rexie = db::build_database().await.unwrap();

        let is_downloaded = db::check_verses_downloaded(&rexie, bible.clone())
            .await
            .unwrap();
        if !is_downloaded {
            let verses = api::download_full_verses(bible.clone())
                .await
                .unwrap()
                .verses;
            let _last_id = db::append_all_verses(&rexie, bible.clone(), verses)
                .await
                .unwrap();
        }

        if self.full_verses.get().is_empty() {
            let full_verses = db::get_full_verses(&rexie, bible.clone()).await.unwrap();
            self.full_verses.set(full_verses);
        }

        rexie.close();
    }

    pub async fn _delete_verses(&self) {
        //let bible = self.selected_bible.get().to_string();

        //let rexie = db::build_database().await.unwrap();
        //db::close_and_delete_db(rexie).await;
    }

    pub async fn load_book_verses(&self) {
        if *self.full_verses_loading.get() {
            return;
        }

        let book_id = self.selected_bible_book.get().book_id;
        if *self.loaded_book.get() != book_id {
            let book_verses = self
                .full_verses
                .get()
                .iter()
                .filter(|v| v.book_id == book_id)
                .cloned()
                .collect::<Vec<VerseItem>>();
            self.loaded_book.set(book_id);
            self.book_verses.set(book_verses);
        }

        self.reset_display_verses();
    }

    pub async fn load_books(&self) {
        let bible = self.selected_bible.get().to_string();

        //check if indexedDB got downloaded the books or not. if got then dont need to call api to download again.
        let rexie = db::build_database().await.unwrap();

        let is_downloaded = db::check_books_downloaded(&rexie, bible.clone())
            .await
            .unwrap();
        if !is_downloaded {
            let books = api::download_books(bible.clone()).await.unwrap().books;
            let _last_id = db::append_all_books(&rexie, bible.clone(), books)
                .await
                .unwrap();
        }

        if self.bible_books.get().is_empty() {
            let bible_books = db::get_books(&rexie, bible.clone()).await.unwrap();
            for b in bible_books.iter() {
                self.bible_books.set(
                    self.bible_books
                        .get()
                        .as_ref()
                        .clone()
                        .into_iter()
                        .chain(Some(create_rc_signal(BibleBookItem {
                            book_id: b.book_id,
                            book_name: b.book_name.clone(),
                            chapters: b.chapters,
                        })))
                        .collect(),
                )
            }
        }

        rexie.close();
    }

    pub fn load_chapters(&self, max: u32) {
        self.chapters.set(Vec::new());
        for n in 1..max + 1 {
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
                    .collect(),
            )
        }
    }

    fn reset_display_verses(&self) {
        if *self.full_verses_loading.get() {
            return;
        }

        let chapter_id = self.selected_bible_chapter.get().id;
        let book_verses = self
            .book_verses
            .get()
            .iter()
            .cloned()
            .filter(|v| v.chapter == chapter_id)
            .collect::<Vec<VerseItem>>();

        self.display_verses.set(Vec::new());
        for v in book_verses.iter() {
            //console::log_1(&"iter".into());
            self.display_verses.set(
                self.display_verses
                    .get()
                    .as_ref()
                    .clone()
                    .into_iter()
                    .chain(Some(create_rc_signal(VerseItem {
                        //id: v.id,
                        book_id: v.book_id,
                        chapter: v.chapter,
                        verse: v.verse,
                        text: v.text.clone(),
                    })))
                    .collect(),
            )
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct BibleBookItem {
    pub book_id: u32,
    pub book_name: String,
    pub chapters: u32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct ChapterItem {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataItem {
    pub name: String,
    pub short_code: String,
    pub language: String,
    pub total_verses: u32, //pub book_id: u32
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct VerseItem {
    //pub id: u32,
    pub book_id: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}
