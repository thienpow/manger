
use serde::{Serialize, Deserialize};
use sycamore::prelude::{RcSignal, create_rc_signal, ScopeRef};
use crate::route::{AppRoutes};
use crate::api;

pub fn initialize(ctx: ScopeRef) {

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    

    match window.local_storage().unwrap() {
        Some(local_storage) => {

            // check dark mode from media query. this will match wil user's pc/phone system setting
            let dark_mode_mq = window.match_media("(prefers-color-scheme: dark)").unwrap().unwrap().matches();
            // then, check it from local_storage
            let dark_mode_ls = local_storage.get_item("dark_mode").unwrap();
            // if got value from local_storage, we will use it, if none, then we use the value from media query
            let is_dark_mode = dark_mode_ls.as_deref() == Some("true") || (dark_mode_ls.is_none() && dark_mode_mq);

            if !is_dark_mode {
                document.body().unwrap().class_list().toggle("light-mode").expect("");
            }

            let bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>> = create_rc_signal(Vec::new());
            let chapters: RcSignal<Vec<RcSignal<ChapterItem>>> = create_rc_signal(Vec::new());
            let verses: RcSignal<Vec<RcSignal<VerseItem>>> = create_rc_signal(Vec::new());
            let loaded_verses: RcSignal<Vec<VerseItem>> = create_rc_signal(Vec::new());
            let loaded_book: RcSignal<i32> = create_rc_signal(0);
            let dark_mode: RcSignal<bool> = create_rc_signal(is_dark_mode);
            let selected_bible_book: RcSignal<BibleBookItem> = create_rc_signal(BibleBookItem {book_id: 0, book_name: "".to_string(), chapters: 0 });
            let selected_bible_chapter: RcSignal<ChapterItem> = create_rc_signal(ChapterItem {id: 0, name: "".to_string() });
            let show_bible_toc: RcSignal<bool> = create_rc_signal(false);
            let pin_bible_toc: RcSignal<bool> = create_rc_signal(true);
            let current_verse_page: RcSignal<i32> = create_rc_signal(0);
            let verse_text_size: RcSignal<i32> = create_rc_signal(12);

            let inner_width: RcSignal<f64> = create_rc_signal(window.inner_width().unwrap().unchecked_into_f64());
            let inner_height: RcSignal<f64> = create_rc_signal(window.inner_height().unwrap().unchecked_into_f64());

            let current_route = CurrentRoute(create_rc_signal(AppRoutes::Home));
            ctx.provide_context(current_route);

            let app_state = AppState {
                bible_books,
                chapters,
                verses,
                loaded_verses,
                loaded_book,
                dark_mode,
                selected_bible_book,
                selected_bible_chapter,
                show_bible_toc,
                pin_bible_toc,
                current_verse_page,
                verse_text_size,
                inner_width, inner_height
            };
            ctx.provide_context(app_state);
        
            
        },
        _ => {}
    }
    
}

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
    pub inner_width: RcSignal<f64>,
    pub inner_height: RcSignal<f64>,
}

impl AppState {

    pub fn toggle_dark_mode(&self) {
        self.dark_mode.set(!*self.dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

        self.set_local_storage("dark_mode", self.dark_mode.get().to_string().as_str());
    }


    pub async fn load_chapter_data(&self) {

        let book_id = self.selected_bible_book.get().book_id;
        if *self.loaded_book.get() != book_id {

            //TODO: check if indexedDB got downloaded the verses or not. if got then dont need to call api to download again.
            let loaded_verses = api::bible::get_book_data("en/kjv".to_string(), book_id).await.unwrap().verses;
            self.loaded_book.set(book_id);
            self.loaded_verses.set(loaded_verses);
        }

        self.reset_verses();
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



    /* 
    private functions below
    */

    fn set_local_storage(&self, key: &str, value: &str) {
        match web_sys::window().unwrap().local_storage().unwrap() {
            Some(local_storage) => {
                local_storage.set_item(key, value).unwrap();
            },
            _ => {}
        }
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
                        chapter: v.chapter,
                        verse: v.verse,
                        text: v.text.clone()
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
