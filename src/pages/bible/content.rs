


use std::{rc::Rc};
use sycamore::prelude::*;
use web_sys::{Event, console, Element};

use crate::{pages::bible::{store::{BibleState, VerseItem}, self}, store::AppState};
use crate::util;

#[derive(Clone)]
struct SelectionFirst(pub RcSignal<VerseItem>);


fn get_marked_index(verse: &Rc<VerseItem>) -> (usize, usize) {

    if verse.book_id == 1 && verse.verse == 1 && verse.chapter == 1 {

        let x1 = 17;
        let x2 = 20;
        (x1, x2)
    } else {
        (0 ,0)
    }

}


#[component]
pub fn BackButton<G: Html>(ctx: Scope) -> View<G> {
    let show_button = ctx.create_signal(false);

    view! { ctx,
        div(class="verse-content-nav-panel", style="border-top-left-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_first_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_previous_page(ctx, 60)
            ) {
                i(class="gg-chevron-left")
            }
        }
    }
}

#[component]
pub fn NextButton<G: Html>(ctx: Scope) -> View<G> {
    let show_button = ctx.create_signal(false);

    view! { ctx,
        div(class="verse-content-nav-panel", style="border-top-right-radius: 12px;",
            on:mouseenter=move |_| show_button.set(true),
            on:mouseleave=move |_| show_button.set(false)
        ) {
            button(
                class=format!("verse-content-nav-button {}", if *show_button.get() && bible::util::check_if_not_last_page() {"nav-button-show"} else {""}),
                on:click=move |_| bible::util::scroll_to_next_page(ctx)
            ) {
                i(class="gg-chevron-right")
            }
        }
    }
}

#[component]
fn VerseItem<G: Html>(ctx: Scope, verse: RcSignal<VerseItem>) -> View<G> {

    let bible_state = ctx.use_context::<BibleState>();
    let verse =  verse.get();
    let prefix = format!("[{}:{}]  ", verse.chapter, verse.verse);

    let mut is_marked = false;
    let verse_text = verse.text.clone();
    let mut verse_text_before_mark: String = "".to_string();
    let mut verse_text_with_mark: String = "".to_string();
    let mut verse_text_after_mark: String = "".to_string();

    let (x1, x2) = get_marked_index(&verse);
    if x2 > 0 {
        is_marked = true;

        verse_text_before_mark = match str::get(verse.text.as_str(), 0..x1) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
        verse_text_with_mark = match str::get(verse.text.as_str(), x1..x2) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
        verse_text_after_mark = match str::get(verse.text.as_str(), x2..) {
            Some(result) => result.to_string(),
            None => "".to_string()
        };
    }

    let on_mousedown = move |_: Event| {
        bible_state.selection_first_verse.set(VerseItem{
            book_id: verse.book_id,
            chapter: verse.chapter,
            verse: verse.verse,
            text: verse.text.clone()
        });
    };
    
    view! { ctx,
        p(on:mousedown=on_mousedown
        ) {
            span(class="", style="-webkit-user-select:none; user-select:none;") {(prefix)}
            (if is_marked {
                view! {ctx,
                    span(){(verse_text_before_mark)}
                    mark() {
                        span(){(verse_text_with_mark)}
                    }
                    span(){(verse_text_after_mark)}
                }
            } else {
                view! {ctx, span(){(verse_text)}}
            })
        }
    }
}


#[component]
pub fn Content<G: Html>(ctx: Scope) -> View<G> {

    let app_state = ctx.use_context::<AppState>();
    let bible_state = ctx.use_context::<BibleState>();
    let verse_content = ctx.create_node_ref();

    //let key_code = ctx.create_signal(0);
    let on_scroll = move |_: Event| {
        
        //let elem = e.current_target().unwrap().unchecked_into::<Element>();
        
        //console::log_1(&format!("{}", elem.scroll_left()).into());
        //key_code.set(key.key_code());
    };

    let get_filtered_verses = ctx.create_memo(|| {
        bible_state
            .verses
            .get()
            .iter()
            //.filter(|v| bible_state.selected_bible_chapter.get().id == v.get().chapter)
            .cloned()
            .collect::<Vec<_>>()
    });

    let on_click = move |_e: Event| {
        
        if *app_state.inner_width.get() <= 420.0 {
            bible_state.show_bible_toc.set(false);
            bible_state.pin_bible_toc.set(false);
        }
    };

    //let result = ctx.create_signal("".to_string());

    let on_mouseup = move |_: Event| {
        let window = web_sys::window().unwrap();
        let selected_text = window.get_selection().unwrap().unwrap().to_string().as_string().unwrap();
        
        if !selected_text.is_empty() {
            //let mut found_index = 0;
            let splited = selected_text.split("\n").into_iter().clone();
            
            for (i, sel_text) in splited.into_iter().enumerate() {
    
                /* do not use... just keep for reference for debugging
                if i == 2 {
                    match sel_text.chars().position(|c| c == ']') {
                        Some(num) => {
                            found_index = num as i32;
                        },
                        _ => {
                            found_index = -1
                        }
                    }
                    
                }
                 */

                if !sel_text.is_empty() {
                    console::log_1(&format!("index: {}, text: {}", i, sel_text).into());
                }
            }

            match bible_state.selection_first_verse.get() {
                verse_item => {
                    
                    let arr = js_sys::Array::new();
                    arr.push(&util::js_array(&["book_id", format!("{}", verse_item.book_id).as_str()]));
                    arr.push(&util::js_array(&["chapter", format!("{}", verse_item.chapter).as_str()]));
                    arr.push(&util::js_array(&["First verse selected", format!("{}", verse_item.verse).as_str()]));
                     
                    console::table_1(&arr);

                }
            };
            
        }


    };

    let on_touchend = move |_: Event| {

        let e = verse_content.get::<DomNode>().unchecked_into::<Element>();
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
    };


    let bible_content_style = ctx.create_memo(|| {
        let inner_width: f64 = *app_state.inner_width.get();
        let inner_height: f64 = *app_state.inner_height.get();
        let mut height =  inner_height-116.0;
        if inner_width <= 738.0 {
            height =  inner_height-128.0;
        } 
        
        format!("font-size:{}pt;height:{}px;", *bible_state.verse_text_size.get(), height)
    });

    view! { ctx,

        article(class="bible-content", style=*bible_content_style.get()) {
            //scroll_to_previous_page
            BackButton()
            //(*key_code.get())
            
            div(class="bible-verse-wrapper"
            ) {
                div(ref=verse_content,
                    id="bible-verse-content", 
                    class="bible-verse-content",
                    on:click=on_click,
                    on:scroll=on_scroll,
                    on:mouseup=on_mouseup,
                    on:touchend=on_touchend
                ) {
                    Keyed {
                        iterable: get_filtered_verses,
                        view: |ctx, verse| view! { ctx,
                            VerseItem(verse)
                        },
                        key: |verse| verse.get().verse,
                    }
                }
            }

            //scroll_to_next_page
            NextButton()
        
        }

    }
}