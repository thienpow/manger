
use serde::{Serialize, Deserialize};
use sycamore::prelude::{RcSignal, create_rc_signal};
use crate::{AppRoutes};


#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub chapters: RcSignal<Vec<RcSignal<ChapterItem>>>,
    pub verses: RcSignal<Vec<RcSignal<VerseItem>>>,
}

impl AppState {
    pub fn reset_chapters(&self, total: i32) {

        self.chapters.set(Vec::new());

        for n in 1..total+1 {
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
        }
        
    }
}

#[derive(Clone)]
pub struct DarkMode(pub RcSignal<bool>);

#[derive(Clone)]
pub struct CurrentRoute(pub RcSignal<AppRoutes>);

#[derive(Clone)]
pub struct LeftMenuOpened(pub RcSignal<bool>);


#[derive(Clone)]
pub struct BackgroundImage(pub RcSignal<String>);

#[derive(Clone)]
pub struct BackgroundVideo(pub RcSignal<String>);



pub struct BibleBook {
    pub id: i32,
    pub name: String,
    pub chapters: i32,
}

#[derive(Clone)]
pub struct SelectedBibleBook(pub RcSignal<BibleBook>);


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
