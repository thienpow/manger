use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::ScopeRef, futures::ScopeSpawnFuture};
use web_sys::console;

use crate::context::{AppState};

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
        TimeoutFuture::new(120).await;

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
                    
        //web_sys::window().unwrap().match_media("(max-width: 420px)").unwrap().unwrap().matches()
        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                
                if e.scroll_left() > 0 {
                    //let pages: f64 = (e.scroll_width() as f64 / e.client_width() as f64).ceil();
                    let previous_page = current_verse_page - 1;
                    app_state.current_verse_page.set(previous_page);
                    e.scroll_with_x_and_y(((previous_page * e.client_width()) + (48 * previous_page)) as f64, 0f64);
                    //console::log_1(&format!("current_page = {}", current_page).into());
                    //console::log_1(&format!("pages = {}", pages).into());
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
                if e.scroll_left() + e.client_width() < e.scroll_width() {
                    //let pages: f32 = (e.scroll_width() as f32 / e.client_width() as f32).ceil();
                    let next_page = current_verse_page + 1;
                    app_state.current_verse_page.set(next_page);
                    e.scroll_with_x_and_y(((next_page * e.client_width()) + (48 * next_page)) as f64, 0f64);
                    //console::log_1(&format!("pages = {}", pages).into());
                    //console::log_1(&format!("current_page = {}", current_page).into());
                }
                //console::log_1(&format!("next_page = {}", next_page).into());
            },
            _ => {}
        }
    });   
}
