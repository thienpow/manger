
use serde::{Serialize, Deserialize};
use sycamore::prelude::{RcSignal, create_rc_signal};
use web_sys::console;
use crate::{AppRoutes};

use crate::api;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>>,
    pub chapters: RcSignal<Vec<RcSignal<ChapterItem>>>,
    pub verses: RcSignal<Vec<RcSignal<VerseItem>>>,
    pub loaded_verses: RcSignal<Vec<VerseItem>>,
    pub loaded_book: RcSignal<i32>,
    pub dark_mode: RcSignal<bool>,
    pub selected_bible_book: RcSignal<BibleBookItem>,
    pub selected_bible_chapter: RcSignal<ChapterItem>,
    pub show_bible_toc: RcSignal<bool>,
    pub pin_bible_toc: RcSignal<bool>,
    pub current_verse_page: RcSignal<i32>,
    pub verse_text_size: RcSignal<i32>,
}

impl AppState {

    pub fn toggle_dark_mode(&self) {
        self.dark_mode.set(!*self.dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

        let local_storage = web_sys::window().unwrap().local_storage().unwrap();
        if let Some(local_storage) = &local_storage {
            //console::log_1(&format!("create_effect").as_str().into());
            local_storage
                .set_item("dark_mode", &*self.dark_mode.get().to_string())
                .unwrap();
        }
    }


    pub async fn load_chapter_data(&self) {
        

        let book_id = self.selected_bible_book.get().book_id;
        if *self.loaded_book.get() != book_id {
            //console::log_1(&"11loaded".into());
            let loaded_verses = api::bible::get_book_data("en/kjv".to_string(), book_id).await.unwrap().verses;
            self.loaded_book.set(book_id);
            self.loaded_verses.set(loaded_verses);
            //console::log_1(&"loaded".into());
        }
        //console::log_1(&"21loaded".into());

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
                        chapter: v.chapter,
                        verse: v.verse,
                        text: v.text.clone()
                    })))
                    .collect()
            )
        }
    }

    pub async fn init_bible_books(&self) {

        if self.bible_books.get().len() == 0 {
            let toc = api::bible::get_toc().await.unwrap_or_default();

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

    pub fn _reset_verses(&self, chapter: i32, total: i32) {

        self.verses.set(Vec::new());

        for n in 1..total+1 {
            self.verses.set(
                self.verses
                    .get()
                    .as_ref()
                    .clone()
                    .into_iter()
                    .chain(Some(create_rc_signal(VerseItem {
                        chapter: chapter,
                        verse: n,
                        text: n.to_string(),
                    })))
                    .collect()
            )
        }
        
    }
}


#[derive(Debug, Default, Clone)]
pub struct DarkMode(pub RcSignal<bool>);


#[derive(Debug, Clone)]
pub struct CurrentRoute(pub RcSignal<AppRoutes>);


#[derive(Clone)]
pub struct LeftMenuOpened(pub RcSignal<bool>);


#[derive(Clone)]
pub struct BackgroundImage(pub RcSignal<String>);

#[derive(Clone)]
pub struct BackgroundVideo(pub RcSignal<String>);




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
    pub chapter: i32,
    pub verse: i32,
    pub text: String
}
