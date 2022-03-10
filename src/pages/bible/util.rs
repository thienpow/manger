use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::ScopeRef, futures::ScopeSpawnFuture};

use crate::store::AppState;

pub fn reload_chapter_data(ctx: ScopeRef) {

    ctx.spawn_future(async move {
        let app_state = ctx.use_context::<AppState>();
        app_state.load_chapter_data().await;

        //evertime loading new chapter data, reset the verse page to 0
        app_state.current_verse_page.set(0);
        scroll_to_previous_page(&ctx, 1000);
    });

}

pub fn scroll_to_selected_book(ctx: ScopeRef) {

    ctx.spawn_future(async move {
        TimeoutFuture::new(60).await;

        let app_state = ctx.use_context::<AppState>();
    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("book-item-{}", app_state.selected_bible_book.get().book_id+1).as_str()) {
            Some(item_below) => {
                item_below.scroll_into_view_with_bool(false);
            },
            _ => {}
        }

    });
    
}

pub fn scroll_to_selected_chapter(ctx: ScopeRef, wait: u32) {

    ctx.spawn_future(async move {
        TimeoutFuture::new(wait).await;

        let app_state = ctx.use_context::<AppState>();
    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("chapter-item-{}", app_state.selected_bible_chapter.get().id+1).as_str()) {
            Some(item_below) => {
                item_below.scroll_into_view_with_bool(false);
            },
            _ => {}
        }
        

    });
    
}


pub fn scroll_to_previous_page(ctx: ScopeRef, wait: u32) {
            
    ctx.spawn_future(async move {
        TimeoutFuture::new(wait).await;

        let app_state = ctx.use_context::<AppState>();
        let current_verse_page = *app_state.current_verse_page.get();

        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                
                if e.scroll_left() > 0 {
                    let previous_page = current_verse_page - 1;
                    app_state.current_verse_page.set(previous_page);
                    e.scroll_with_x_and_y(((previous_page * e.client_width()) as f64 + (47.5f64 * previous_page as f64)) as f64, 0f64);
                }
                
            },
            _ => {}
        }

        
    });
}

pub fn scroll_to_next_page(ctx: ScopeRef) {
        
    ctx.spawn_future(async move {
        TimeoutFuture::new(60).await;

        let app_state = ctx.use_context::<AppState>();
        let current_verse_page = *app_state.current_verse_page.get();
                    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                if e.scroll_width() - e.scroll_left() > e.client_width() + 10  {
                    let next_page = current_verse_page + 1;
                    app_state.current_verse_page.set(next_page);
                    e.scroll_with_x_and_y(((next_page * e.client_width()) as f64 + (47.5f64 * next_page as f64)) as f64, 0f64);
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

