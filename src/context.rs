
use serde::{Serialize, Deserialize};
use sycamore::prelude::{RcSignal, create_rc_signal, ScopeRef, ReadSignal, Scope};
use web_sys::console;
use crate::{AppRoutes};


#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub bible_books: RcSignal<Vec<RcSignal<BibleBookItem>>>,
    pub chapters: RcSignal<Vec<RcSignal<ChapterItem>>>,
    pub verses: RcSignal<Vec<RcSignal<VerseItem>>>,
    pub dark_mode: RcSignal<bool>,
    pub selected_bible_book: RcSignal<BibleBookItem>,
    pub selected_bible_chapter: RcSignal<ChapterItem>,
}

impl AppState {

    pub fn toggle_dark_mode(&self) {
        self.dark_mode.set(!*self.dark_mode.get());

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().class_list().toggle("light-mode").expect("");

        let local_storage = web_sys::window().unwrap().local_storage().unwrap();
        if let Some(local_storage) = &local_storage {
            console::log_1(&format!("create_effect").as_str().into());
            local_storage
                .set_item("dark_mode", &*self.dark_mode.get().to_string())
                .unwrap();
        }
    }

    pub fn init_bible_books(&self, bible_books: Vec<BibleBookItem>) {

        self.bible_books.set(Vec::new());
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
                        chapters: b.chapters
                    })))
                    .collect()
            )
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

//{"chapter":1,"verse":1,"text":"In the beginning God created the heaven and the earth."},
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct VerseItem {
    pub chapter: i32,
    pub verse: i32,
    pub text: String
}
