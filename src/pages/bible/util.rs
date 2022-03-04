use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::ScopeRef, futures::ScopeSpawnFuture};

use crate::context::{AppState, ChapterItem};


pub fn scroll_to_selected(ctx: ScopeRef) {

    ctx.spawn_future(async move {
        TimeoutFuture::new(60).await;

        let app_state = ctx.use_context::<AppState>();
    
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
    
        match document.get_element_by_id("book_list") {
            Some(list) => {
                list.set_scroll_top(((app_state.selected_bible_book.get().book_id-1) * 53)+15);
            },
            _ => {}
        }
        
        match document.get_element_by_id("chapter_list") {
            Some(list) => {
                let id = app_state.selected_bible_chapter.get().id;
                if id > 0 {
                    list.set_scroll_top(((id-1) * 53)+15);
                } else {
                    list.set_scroll_top(53+15);
                }
            },
            _ => {}
        }
    });
    
}
