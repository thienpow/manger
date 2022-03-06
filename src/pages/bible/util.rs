use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::ScopeRef, futures::ScopeSpawnFuture};
use web_sys::console;

use crate::context::{AppState};

pub fn reload_chapter_data(ctx: ScopeRef) {

    ctx.spawn_future(async move {
        let app_state = ctx.use_context::<AppState>();
        app_state.load_chapter_data().await;
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