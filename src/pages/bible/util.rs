use std::{cell::RefCell, rc::Rc};

use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::Scope, futures::ScopeSpawnLocal};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::pages::bible::store::BibleState;

pub fn reload_chapter_data(ctx: Scope) {
    ctx.spawn_local(async move {
        let app_state = ctx.use_context::<BibleState>();
        app_state.load_chapter_data().await;

        //evertime loading new chapter data, reset the verse page to 0
        app_state.current_verse_page.set(0);
        scroll_to_previous_page(ctx, 1000);
    });
}

/*
    TODO: see if can use request_animation_frame method

    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
       
        let _ = f.borrow_mut().take();

    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

 */


pub fn scroll_to_selected_book(ctx: Scope, wait: u32) {

    ctx.spawn_local(async move {
        TimeoutFuture::new(wait).await;

        let app_state = ctx.use_context::<BibleState>();
        let target_id = app_state.selected_bible_book.get().book_id+1;
        
        match web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("book-item-{}", target_id).as_str()) {
            Some(item_below) => {
                item_below.scroll_into_view_with_bool(false);
            },
            _ => {}
        }
    });
}

pub fn scroll_to_selected_chapter(ctx: Scope, wait: u32) {

    ctx.spawn_local(async move {
        TimeoutFuture::new(wait).await;

        let app_state = ctx.use_context::<BibleState>();
        let target_id = app_state.selected_bible_chapter.get().id+1;
    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("chapter-item-{}", target_id).as_str()) {
            Some(item_below) => {
                item_below.scroll_into_view_with_bool(false);
            },
            _ => {}
        }
    });
    
}


pub fn scroll_to_previous_page(ctx: Scope, wait: u32) {
    ctx.spawn_local(async move {
        TimeoutFuture::new(wait).await;

        let bible_state = ctx.use_context::<BibleState>();
        let current_verse_page = *bible_state.current_verse_page.get();

        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                
                if e.scroll_left() > 0 {
                    let previous_page = current_verse_page - 1;
                    bible_state.current_verse_page.set(previous_page);
                    let x = (previous_page * e.client_width()) as f64 + (48.0 * previous_page as f64);
                    bible_state.current_verse_scroll_x.set(x);
                    e.scroll_with_x_and_y(x, 0.0);
                }
                
            },
            _ => {}
        }
    });
}

pub fn scroll_to_next_page(ctx: Scope) {
    ctx.spawn_local(async move {
        TimeoutFuture::new(60).await;

        let bible_state = ctx.use_context::<BibleState>();
        let current_verse_page = *bible_state.current_verse_page.get();
                    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                if e.scroll_width() - e.scroll_left() > e.client_width()  {
                    let next_page = current_verse_page + 1;
                    bible_state.current_verse_page.set(next_page);
                    let x = (next_page * e.client_width()) as f64 + (48.0 * next_page as f64);
                    bible_state.current_verse_scroll_x.set(x);
                    e.scroll_with_x_and_y(x, 0.0);
                }
            },
            _ => {}
        }
    });   
}


pub fn check_if_not_first_page() -> bool {
    match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
        Some(e) => {
            if e.scroll_left() > 0 {
                true 
            }
            else {
                false
            }
        },
        _ => {true}
    }
}
pub fn check_if_not_last_page() -> bool {
    match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
        Some(e) => {
            if e.scroll_width() - e.scroll_left() > e.client_width() + 10  {
                true 
            }
            else {
                false
            }
        },
        _ => {true}
    }
}

