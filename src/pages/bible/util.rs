use gloo_timers::future::TimeoutFuture;
use sycamore::{prelude::ScopeRef, futures::ScopeSpawnFuture};
use web_sys::{console};

use crate::pages::bible::{store::BibleState};

pub fn reload_chapter_data(ctx: ScopeRef) {
    ctx.spawn_future(async move {
        let app_state = ctx.use_context::<BibleState>();
        app_state.load_chapter_data().await;

        //evertime loading new chapter data, reset the verse page to 0
        app_state.current_verse_page.set(0);
        scroll_to_previous_page(&ctx, 1000);
    });
}

pub fn scroll_to_selected_book(ctx: ScopeRef, wait: u32) {
    ctx.spawn_future(async move {
        TimeoutFuture::new(wait).await;
        let app_state = ctx.use_context::<BibleState>();
        
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
        let app_state = ctx.use_context::<BibleState>();
        match web_sys::window().unwrap().document().unwrap().get_element_by_id(format!("chapter-item-{}", app_state.selected_bible_chapter.get().id+1).as_str()) {
            Some(item_below) => {
                item_below.scroll_into_view_with_bool(false);
            },
            _ => {}
        }
    });
}

// scroll to the near page, by calc 1/3 of width to decide next or previous
pub fn scroll_to_near_page(ctx: ScopeRef, wait: u32) {
    ctx.spawn_future(async move {
        TimeoutFuture::new(wait).await;

        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                let bible_state = ctx.use_context::<BibleState>();

                let previous_scroll_page = (*bible_state.current_verse_scroll_x.get() / (e.client_width() as f64 + 48.0)).floor();
                let current_scroll_page_actual = e.scroll_left() as f64 / (e.client_width() as f64 + 48.0);
                
                let mut page_to_scroll = *bible_state.current_verse_page.get() as f64;
                let current_scroll_page_diff = current_scroll_page_actual - previous_scroll_page;
                if current_scroll_page_diff > 1.2/10.0 {
                    page_to_scroll = previous_scroll_page + current_scroll_page_diff.ceil();
                } else if current_scroll_page_diff < -1.2/10.0 {
                    page_to_scroll = previous_scroll_page + current_scroll_page_diff.floor();
                }
                
                bible_state.current_verse_page.set(page_to_scroll as i32);
                let x = (page_to_scroll * e.client_width() as f64) as f64 + (48.0 * page_to_scroll as f64);
                bible_state.current_verse_scroll_x.set(x);
                e.scroll_with_x_and_y(x, 0.0);
                
            },
            _ => {}
        }
        
    });
}

pub fn scroll_to_previous_page(ctx: ScopeRef, wait: u32) {
    ctx.spawn_future(async move {
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

pub fn scroll_to_next_page(ctx: ScopeRef) {
    ctx.spawn_future(async move {
        TimeoutFuture::new(60).await;

        let bible_state = ctx.use_context::<BibleState>();
        let current_verse_page = *bible_state.current_verse_page.get();
                    
        match web_sys::window().unwrap().document().unwrap().get_element_by_id("bible-verse-content") {
            Some(e) => {
                if e.scroll_width() - e.scroll_left() > e.client_width() + 10  {
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

