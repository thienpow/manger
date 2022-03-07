
use sycamore::prelude::*;
use sycamore_router::{Route};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

mod api;
mod pages;
mod context;
mod svg;
mod components;
mod global;
use crate::components::{background::Background};
use crate::context::{CurrentRoute, LeftMenuOpened, BibleBookItem, AppState, ChapterItem, VerseItem};
use crate::pages::app::App;

#[derive(Debug, Clone, Route)]
pub  enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/bible")]
    Bible,
    #[to("/love")]
    Love,
    #[to("/community")]
    Community,
    #[to("/chat/<id>")]
    Chat(String),
    #[to("/profile")]
    Profile,
    #[not_found]
    NotFound,
}

fn main() {
    /*
        function resizeListener() {
            heightOutput.textContent = window.innerHeight;
            widthOutput.textContent = window.innerWidth;
        }

        window.addEventListener("resize", resizeListener);
    */

    sycamore::render(|ctx| {

        
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        let local_storage = window.local_storage().unwrap();
        
        // Get dark mode from media query.
        let dark_mode_mq = window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap()
            .matches();
            
        let is_dark_mode = if let Some(local_storage) = &local_storage {
            let dark_mode_ls = local_storage.get_item("dark_mode").unwrap();
            dark_mode_ls.as_deref() == Some("true") || (dark_mode_ls.is_none() && dark_mode_mq)
        } else {
            dark_mode_mq
        };
        
        if !is_dark_mode {
            document.body().unwrap().class_list().toggle("light-mode").expect("");
        }

        let left_menu_opened = LeftMenuOpened(create_rc_signal(false));
        ctx.provide_context(left_menu_opened);

        /* 
        let window_resize_closure = Closure::wrap(Box::new(move |_: web_sys::UiEvent| {

            let mut height: f64 =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-188.0;
            let width: f64 = web_sys::window().unwrap().inner_width().unwrap().unchecked_into_f64();

            //console::log_1(&format!("window_resize_closure").as_str().into());

            let chat_content = web_sys::window().unwrap().document().unwrap().get_element_by_id("chat-content");
            if chat_content.is_some() {
                let chat_content = chat_content.unwrap();
                
                if width <= 540.0 {
                    height =  web_sys::window().unwrap().inner_height().unwrap().unchecked_into_f64()-130.0;
                    
                }
                chat_content.set_attribute("style", format!("height:{}px;", height).as_str()).unwrap();
            }
            
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback("resize", window_resize_closure.as_ref().unchecked_ref()).unwrap();
        window_resize_closure.forget();
        */



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
            current_verse_page
        };
        ctx.provide_context(app_state);


        view! { ctx, 
            Background()
            //ToggleMode()
            App()
        }
    });
}
